/* See the "Debug Messages" section of the NO$GBA help for more detail. */
/* (the website is outdated, view it in the actual app) */
use core::arch::asm;
use voladdress::*;

const CHAR_OUT: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x04FFFA1C) };

pub fn print (s: &str) {
    for b in s.bytes() {
        CHAR_OUT.write(b);
    }
}

// Works in both ARM and THUMB mode
pub macro breakpoint () {
    unsafe {
        asm!(
            "mov r11, r11",
            options(nomem, preserves_flags, nostack)
        );
    }
}
