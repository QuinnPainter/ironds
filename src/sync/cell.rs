use core::{
    arch::asm,
    cell::UnsafeCell,
    fmt::Debug,
    mem::{align_of, size_of},
    num::{NonZeroI32, NonZeroI16, NonZeroI8, NonZeroU32, NonZeroU16, NonZeroU8},
};

#[repr(transparent)]
pub struct NdsCell<T>(UnsafeCell<T>);
unsafe impl<T> Send for NdsCell<T> {}
unsafe impl<T> Sync for NdsCell<T> {}

impl<T> NdsCell<T>
where T: NdsCellSafe {
    #[inline]
    #[must_use]
    pub const fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }

    #[inline]
    #[must_use]
    pub const fn get_ptr(&self) -> *mut T {
        self.0.get()
    }

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

pub unsafe trait NdsCellSafe: Copy {}

unsafe impl NdsCellSafe for bool {}
