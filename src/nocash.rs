/* See the "Debug Messages" section of the NO$GBA help for more detail. */
/* (the website is outdated, view it in the actual app) */
use core::arch::asm;
use core::ptr::write_volatile;
use crate::addr;

// Works in NO$GBA, melonDS and DeSmuME
#[instruction_set(arm::a32)]
#[inline(never)]
pub fn print (s: &str) {
    for chunk in s.as_bytes().chunks(100) {
        unsafe {
            asm!(
                ".arm",
                "ldr r0, =2f",
                "add r3, r0, r2", //
                "mov r4, #0",     // insert 0 terminator at end of string
                "strb r4, [r3]",  //
                "ldr r3, =__aeabi_memcpy", // copy the input string into the .space below
                "bl 3f", // jumps to "bx r3"
                "mov r12, r12",
                "b 4f", // f = local label is forwards (llvm bug prevents using labels 0 and 1?)
                ".hword 0x6464", // magic number
                ".hword 0", // flags?
                "2:",
                ".space 101", // extra byte for 0 terminator
                "3:",
                "bx r3", // separate bx since arm7 doesn't have blx
                "4:",
                ".align",
                in("r1") chunk as *const [u8] as *const u8, // inline version of ".as_ptr()"
                in("r2") chunk.len(),
                out("r4") _,
                clobber_abi("C"),
            );
        }
    }
}

// Works in NO$GBA and (soon?) melonDS
// you should probably use "print" instead, as it is more compatible. this is just included for posterity.
pub fn print2 (s: &str) {
    for b in s.bytes() {
        // this reg is really 8 bit in no$gba, but melonds won't accept it unless it's treated as 32 bit
        unsafe { write_volatile(addr::NOCASH_CHAROUT as *mut u32, b as u32); }
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
