//! Wrappers for the builtin BIOS functions.
//! 
//! They should generally not be called directly, but through the wrappers provided in other modules.  
//! See <https://problemkaputt.de/gbatek.htm#biosfunctions>
// todo: is it okay to allow these to inline? is rust smart enough to inline for thumb code, but not for arm?

use core::arch::asm;

/// Wait for any interrupt, then continue.
#[inline(never)]
#[instruction_set(arm::t32)]
pub fn halt() {
    unsafe {
        asm!(
            "swi 0x06",
            lateout("r0") _,
            lateout("r1") _,
            lateout("r2") _,
            lateout("r3") _,
        );
    }
}

/// Wait for one of the specified interrupts, then continue.
#[inline(never)]
#[instruction_set(arm::t32)]
pub fn intr_wait(r0: u32, r1: u32, r2: u32) {
    unsafe {
        asm!(
            "swi 0x04",
            inlateout("r0") r0 => _,
            inlateout("r1") r1 => _,
            inlateout("r2") r2 => _,
            lateout("r3") _,
        );
    }
}

/// Wait for the VBlank interrupt, then continue.
#[inline(never)]
#[instruction_set(arm::t32)]
pub fn vblank_intr_wait() {
    unsafe {
        asm!(
            "swi 0x05",
            lateout("r0") _,
            lateout("r1") _,
            lateout("r2") _,
            lateout("r3") _,
        );
    }
}
