//! The startup runtime that runs before the `main` function.
//! 
//! Also known as "rt0". Does things like initialise variables, copy your stuff
//! into ITCM / DTCM / IWRAM, and initialise hardware registers.

use core::arch::global_asm;
use core::panic;

#[cfg(feature = "arm9")]
global_asm! {
    include_str!("init_arm9.s"),
    options(raw)
}

#[cfg(feature = "arm7")]
global_asm! {
    include_str!("init_arm7.s"),
    options(raw)
}

#[no_mangle]
#[doc(hidden)]
pub fn return_from_main() -> ! {
    panic!("returned from main");
}

// pad out the secure area so the cart isn't encrypted
// this could be moved to the linkerscript, not sure if there's value having it here
#[doc(hidden)]
#[cfg(feature = "arm9")]
#[link_section = ".secure"]
pub static SECURE_PADDING: [u8; 2048] = [0; 2048];
