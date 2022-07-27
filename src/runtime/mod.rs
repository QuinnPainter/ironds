use core::arch::global_asm;
use core::panic;

global_asm! {
    include_str!("init.s"),
    options(raw)
}

#[no_mangle]
pub fn return_from_main() -> ! {
    panic!("returned from main");
}

// pad out the secure area so the cart isn't encrypted
#[link_section = ".secure"]
#[no_mangle]
pub static SECURE_PADDING: [u8; 2048] = [0; 2048];
