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
pub fn return_from_main() -> ! {
    panic!("returned from main");
}

// pad out the secure area so the cart isn't encrypted
#[cfg(feature = "arm9")]
#[link_section = ".secure"]
#[no_mangle]
pub static SECURE_PADDING: [u8; 2048] = [0; 2048];
