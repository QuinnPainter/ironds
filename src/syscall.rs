//! Wrappers for the builtin BIOS functions.

use core::arch::asm;

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
