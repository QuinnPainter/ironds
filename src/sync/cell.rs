// Uses components from rust-console/gba, under a Zlib license:
// Copyright (c) 2019 Daniel "Lokathor" Gee.
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
// Permission is granted to anyone to use this software for any purpose, including commercial
// applications, and to alter it and redistribute it freely, subject to the following restrictions:
// 1. The origin of this software must not be misrepresented; you must not claim that you wrote
// the original software. If you use this software in a product, an acknowledgment in the
// product documentation would be appreciated but is not required.
// 2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source distribution.

use core::{
    arch::asm,
    cell::UnsafeCell,
    fmt::Debug,
    mem::{align_of, size_of},
    num::{NonZeroI32, NonZeroI16, NonZeroI8, NonZeroU32, NonZeroU16, NonZeroU8},
};

/// A NDS-specific wrapper around Rust's [`UnsafeCell`](core::cell::UnsafeCell)
/// type.
/// 
/// Allows data to be safely shared between the main program and the interrupt handler.
/// 
/// Supports any data type that implements the [`NdsCellSafe`] marker trait.
///
/// Based on [`GbaCell`](https://docs.rs/gba/latest/gba/gba_cell/struct.GbaCell.html)
/// from rust-console/gba.
/// 
/// ## Safety Logic
///
/// * LLVM thinks that ARMv4T / ARMv5TE only supports atomic operations via special atomic
///   support library functions. This is true for the "complex" atomic ops like
///   "fetch-add", but for individual load or store ops this is overkill.
/// * If you directly write an Acquire/load, Release/store, or a Relaxed op with
///   an associated `compiler_fence`, then LLVM does generate correct code.
///   However, it will have very sub-optimal performance. LLVM will generate
///   calls to the mythical atomic support library, when it should just directly
///   use an `ldr` or `str` instruction.
/// * In response to this LLVM nonsense, the `NdsCell` type just uses inline
///   assembly to perform all accesses to the contained data.
/// * When LLVM sees inline assembly, it is forced to defensively act as if the
///   inline assembly might have done *anything* legally possible using the
///   pointer and value provided to the inline assembly. This includes that the
///   inline assembly *might* call the atomic support library to access the
///   pointer's data using an atomic load or store. So LLVM has to treat the
///   inline assembly as an atomic sync point.
/// * However, inside the inline asm block we actually just use the single load
///   or store op that we wanted.
#[repr(transparent)]
pub struct NdsCell<T>(UnsafeCell<T>);
unsafe impl<T> Send for NdsCell<T> {}
unsafe impl<T> Sync for NdsCell<T> {}

impl<T> NdsCell<T>
where T: NdsCellSafe {
    /// Wraps a value in a new `NdsCell`.
    #[inline]
    #[must_use]
    pub const fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }

    /// Gets a pointer to the inner data.
    ///
    /// The rules for this pointer work just like with [`UnsafeCell`].
    #[inline]
    #[must_use]
    pub const fn get_ptr(&self) -> *mut T {
        self.0.get()
    }

    /// Reads the value.
    #[inline]
    #[must_use]
    pub fn read(&self) -> T {
        match (size_of::<T>(), align_of::<T>()) {
            (4, 4) => unsafe {
                let val: u32;
                asm!(
                    "ldr {r}, [{addr}]",
                    r = lateout(reg) val,
                    addr = in(reg) self.get_ptr(),
                    options(readonly, preserves_flags, nostack)
                );
                core::mem::transmute_copy(&val)
            },
            (2, 2) => unsafe {
                let val: u16;
                asm!(
                    "ldrh {r}, [{addr}]",
                    r = lateout(reg) val,
                    addr = in(reg) self.get_ptr(),
                    options(readonly, preserves_flags, nostack)
                );
                core::mem::transmute_copy(&val)
            },
            (1, 1) => unsafe {
                let val: u8;
                asm!(
                    "ldrb {r}, [{addr}]",
                    r = lateout(reg) val,
                    addr = in(reg) self.get_ptr(),
                    options(readonly, preserves_flags, nostack)
                );
                core::mem::transmute_copy(&val)
            },
            _ => {
                unimplemented!()
            }
        }
    }

    /// Writes a new value.
    #[inline]
    pub fn write(&self, val: T) {
        match (size_of::<T>(), align_of::<T>()) {
            (4, 4) => unsafe {
                let u: u32 = core::mem::transmute_copy(&val);
                asm!(
                    "str {v}, [{addr}]",
                    v = in(reg) u,
                    addr = in(reg) self.get_ptr(),
                    options(preserves_flags, nostack)
                );
            },
            (2, 2) => unsafe {
                let u: u16 = core::mem::transmute_copy(&val);
                asm!(
                    "strh {v}, [{addr}]",
                    v = in(reg) u,
                    addr = in(reg) self.get_ptr(),
                    options(preserves_flags, nostack)
                );
            },
            (1, 1) => unsafe {
                let u: u8 = core::mem::transmute_copy(&val);
                asm!(
                    "strb {v}, [{addr}]",
                    v = in(reg) u,
                    addr = in(reg) self.get_ptr(),
                    options(preserves_flags, nostack)
                );
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

impl<T> NdsCell<T>
where T: NdsCellSwapSafe {
    /// Writes a new value in, and returns the old value.
    /// 
    /// Different to just doing a `read` followed by a `write`, since this function
    /// performs the operation atomically using a single `swp` instruction.
    /// 
    /// This makes the function useful for operations that must be atomic, such as
    /// cross-thread communication with mutexes.
    #[cfg_attr(feature = "arm9", link_section = ".itcm.nds_cell_swap")]
    #[must_use]
    #[instruction_set(arm::a32)]
    pub fn swap(&self, val: T) -> T {
        match (size_of::<T>(), align_of::<T>()) {
            (4, 4) => unsafe {
                let u: u32 = core::mem::transmute_copy(&val);
                let o: u32;
                asm!(
                    "swp {o}, {u}, [{addr}]",
                    u = in(reg) u,
                    o = lateout(reg) o,
                    addr = in(reg) self.get_ptr(),
                    options(preserves_flags, nostack)
                );
                core::mem::transmute_copy(&o)
            },
            (1, 1) => unsafe {
                let u: u8 = core::mem::transmute_copy(&val);
                let o: u8;
                asm!(
                    "swpb {o}, {u}, [{addr}]",
                    u = in(reg) u,
                    o = lateout(reg) o,
                    addr = in(reg) self.get_ptr(),
                    options(preserves_flags, nostack)
                );
                core::mem::transmute_copy(&o)
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

impl<T> Debug for NdsCell<T>
where T: NdsCellSafe + Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as Debug>::fmt(&self.read(), f)
    }
}

/// Marker trait bound for the methods of [`NdsCell`].
///
/// When a type implements this trait it indicates that the type can be loaded
/// from a pointer in a single instruction. Also it can be stored to a pointer
/// in a single instruction.
///
/// The exact pair of load/store instructions used will depend on the type's
/// size (`ldr`/`str`, `ldrh`/`strh`, or `ldrb`/`strb`).
///
/// ## Safety
/// The type must fit in a single register and have an alignment equal to its
/// size. Generally that means it should be one of:
///
/// * an 8, 16, or 32 bit integer
/// * a function pointer
/// * a data pointer to a sized type
/// * an optional non-null pointer (to function or sized data)
/// * a `repr(transparent)` newtype over one of the above
pub unsafe trait NdsCellSafe: Copy {}

unsafe impl NdsCellSafe for bool {}
unsafe impl NdsCellSafe for char {}
unsafe impl NdsCellSafe for i16 {}
unsafe impl NdsCellSafe for i32 {}
unsafe impl NdsCellSafe for i8 {}
unsafe impl NdsCellSafe for NonZeroI16 {}
unsafe impl NdsCellSafe for NonZeroI32 {}
unsafe impl NdsCellSafe for NonZeroI8 {}
unsafe impl NdsCellSafe for NonZeroU16 {}
unsafe impl NdsCellSafe for NonZeroU32 {}
unsafe impl NdsCellSafe for NonZeroU8 {}
unsafe impl NdsCellSafe for Option<bool> {}
unsafe impl NdsCellSafe for Option<char> {}
unsafe impl NdsCellSafe for Option<NonZeroI16> {}
unsafe impl NdsCellSafe for Option<NonZeroI32> {}
unsafe impl NdsCellSafe for Option<NonZeroI8> {}
unsafe impl NdsCellSafe for Option<NonZeroU16> {}
unsafe impl NdsCellSafe for Option<NonZeroU32> {}
unsafe impl NdsCellSafe for Option<NonZeroU8> {}
unsafe impl NdsCellSafe for u16 {}
unsafe impl NdsCellSafe for u32 {}
unsafe impl NdsCellSafe for u8 {}

/// Marker trait bound for the `swap` method of [`NdsCell`].
///
/// When implemented in conjunction with [`NdsCellSafe`], this allows a type to use
/// the `swap` function in [`NdsCell`].
/// 
/// A seperate trait is required for this because this function uses the `swp`
/// and `swpb` ARM instructions, which are only available for 32 bit and 8 bit values.
///
/// Therefore, any 32 bit / 8 bit types (u8, u32, function pointers, etc.) should
/// impl both NdsCellSafe and NdsCellSwapSafe, while 16 bit types should impl
/// only NdsCellSafe.
pub unsafe trait NdsCellSwapSafe: NdsCellSafe {}

unsafe impl NdsCellSwapSafe for bool {}
unsafe impl NdsCellSwapSafe for char {}
unsafe impl NdsCellSwapSafe for i32 {}
unsafe impl NdsCellSwapSafe for i8 {}
unsafe impl NdsCellSwapSafe for NonZeroI32 {}
unsafe impl NdsCellSwapSafe for NonZeroI8 {}
unsafe impl NdsCellSwapSafe for NonZeroU32 {}
unsafe impl NdsCellSwapSafe for NonZeroU8 {}
unsafe impl NdsCellSwapSafe for Option<bool> {}
unsafe impl NdsCellSwapSafe for Option<char> {}
unsafe impl NdsCellSwapSafe for Option<NonZeroI32> {}
unsafe impl NdsCellSwapSafe for Option<NonZeroI8> {}
unsafe impl NdsCellSwapSafe for Option<NonZeroU32> {}
unsafe impl NdsCellSwapSafe for Option<NonZeroU8> {}
unsafe impl NdsCellSwapSafe for u32 {}
unsafe impl NdsCellSwapSafe for u8 {}
