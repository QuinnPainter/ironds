use core::ptr::{read_volatile, write_volatile};
use crate::addr;

// https://www.problemkaputt.de/gbatek.htm#dsmemorycontrolvram
pub mod vram_type {
    #![allow(non_camel_case_types)]
    const VRAM_ENABLE: u8 = 1 << 7;

    #[repr(u8)]
    pub enum A { // 128k
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_0 = 1 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_1 = 1 | (1 << 3) | VRAM_ENABLE,
        MAIN_BG_2 = 1 | (2 << 3) | VRAM_ENABLE,
        MAIN_BG_3 = 1 | (3 << 3) | VRAM_ENABLE,
        MAIN_OBJ_0 = 2 | (0 << 3) | VRAM_ENABLE,
        MAIN_OBJ_1 = 2 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_0 = 3 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_1 = 3 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_2 = 3 | (2 << 3) | VRAM_ENABLE,
        TEXTURE_3 = 3 | (3 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum B { // 128k (same options as A)
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_0 = 1 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_1 = 1 | (1 << 3) | VRAM_ENABLE,
        MAIN_BG_2 = 1 | (2 << 3) | VRAM_ENABLE,
        MAIN_BG_3 = 1 | (3 << 3) | VRAM_ENABLE,
        MAIN_OBJ_0 = 2 | (0 << 3) | VRAM_ENABLE,
        MAIN_OBJ_1 = 2 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_0 = 3 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_1 = 3 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_2 = 3 | (2 << 3) | VRAM_ENABLE,
        TEXTURE_3 = 3 | (3 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum C { // 128k
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_0 = 1 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_1 = 1 | (1 << 3) | VRAM_ENABLE,
        MAIN_BG_2 = 1 | (2 << 3) | VRAM_ENABLE,
        MAIN_BG_3 = 1 | (3 << 3) | VRAM_ENABLE,
        ARM7_0 = 2 | (0 << 3) | VRAM_ENABLE,
        ARM7_1 = 2 | (1 << 3) | VRAM_ENABLE,
        SUB_BG = 4 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_0 = 3 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_1 = 3 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_2 = 3 | (2 << 3) | VRAM_ENABLE,
        TEXTURE_3 = 3 | (3 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum D { // 128k
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_0 = 1 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_1 = 1 | (1 << 3) | VRAM_ENABLE,
        MAIN_BG_2 = 1 | (2 << 3) | VRAM_ENABLE,
        MAIN_BG_3 = 1 | (3 << 3) | VRAM_ENABLE,
        ARM7_0 = 2 | (0 << 3) | VRAM_ENABLE,
        ARM7_1 = 2 | (1 << 3) | VRAM_ENABLE,
        SUB_OBJ = 4 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_0 = 3 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_1 = 3 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_2 = 3 | (2 << 3) | VRAM_ENABLE,
        TEXTURE_3 = 3 | (3 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum E { // 64k
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_0 = 1 | (0 << 3) | VRAM_ENABLE, // only occupies first half of slot 0
        MAIN_OBJ_0 = 2 | (0 << 3) | VRAM_ENABLE, // only occupies first half of slot 0
        MAIN_BG_EX_PAL_0_to_3 = 4 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_0_to_3 = 3 | (0 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum F { // 16k
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_0_0 = 1 | (0 << 3) | VRAM_ENABLE, // 0x06000000
        MAIN_BG_0_1 = 1 | (1 << 3) | VRAM_ENABLE, // 0x06004000
        MAIN_BG_0_2 = 1 | (2 << 3) | VRAM_ENABLE, // 0x06010000
        MAIN_BG_0_3 = 1 | (3 << 3) | VRAM_ENABLE, // 0x06014000
        MAIN_OBJ_0_0 = 2 | (0 << 3) | VRAM_ENABLE, // 0x06400000
        MAIN_OBJ_0_1 = 2 | (1 << 3) | VRAM_ENABLE, // 0x06404000
        MAIN_OBJ_0_2 = 2 | (2 << 3) | VRAM_ENABLE, // 0x06410000
        MAIN_OBJ_0_3 = 2 | (3 << 3) | VRAM_ENABLE, // 0x06414000
        MAIN_BG_EX_PAL_0_to_1 = 4 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_EX_PAL_2_to_3 = 4 | (1 << 3) | VRAM_ENABLE,
        MAIN_OBJ_EX_PAL = 5 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_0 = 3 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_1 = 3 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_4 = 3 | (2 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_5 = 3 | (3 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum G { // 16k (same options as F)
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_0_0 = 1 | (0 << 3) | VRAM_ENABLE, // 0x06000000
        MAIN_BG_0_1 = 1 | (1 << 3) | VRAM_ENABLE, // 0x06004000
        MAIN_BG_0_2 = 1 | (2 << 3) | VRAM_ENABLE, // 0x06010000
        MAIN_BG_0_3 = 1 | (3 << 3) | VRAM_ENABLE, // 0x06014000
        MAIN_OBJ_0_0 = 2 | (0 << 3) | VRAM_ENABLE, // 0x06400000
        MAIN_OBJ_0_1 = 2 | (1 << 3) | VRAM_ENABLE, // 0x06404000
        MAIN_OBJ_0_2 = 2 | (2 << 3) | VRAM_ENABLE, // 0x06410000
        MAIN_OBJ_0_3 = 2 | (3 << 3) | VRAM_ENABLE, // 0x06414000
        MAIN_BG_EX_PAL_0_to_1 = 4 | (0 << 3) | VRAM_ENABLE,
        MAIN_BG_EX_PAL_2_to_3 = 4 | (1 << 3) | VRAM_ENABLE,
        MAIN_OBJ_EX_PAL = 5 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_0 = 3 | (0 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_1 = 3 | (1 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_4 = 3 | (2 << 3) | VRAM_ENABLE,
        TEXTURE_PAL_5 = 3 | (3 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum H { // 32k
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        SUB_BG = 1 | (0 << 3) | VRAM_ENABLE,
        SUB_BG_EX_PAL = 2 | (0 << 3) | VRAM_ENABLE,
    }

    #[repr(u8)]
    pub enum I { // 16k
        LCDC = 0 | (0 << 3) | VRAM_ENABLE,
        SUB_BG = 1 | (0 << 3) | VRAM_ENABLE,
        SUB_OBJ = 2 | (0 << 3) | VRAM_ENABLE,
        SUB_OBJ_EX_PAL = 3 | (0 << 3) | VRAM_ENABLE,
    }
}

// todo: deduplicate these functions somehow. proc macro?
#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_a(vtype: vram_type::A) {
    unsafe { write_volatile(addr::VRAMCNT_A as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_b(vtype: vram_type::B) {
    unsafe { write_volatile(addr::VRAMCNT_B as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_c(vtype: vram_type::C) {
    unsafe { write_volatile(addr::VRAMCNT_C as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_d(vtype: vram_type::D) {
    unsafe { write_volatile(addr::VRAMCNT_D as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_e(vtype: vram_type::E) {
    unsafe { write_volatile(addr::VRAMCNT_E as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_f(vtype: vram_type::F) {
    unsafe { write_volatile(addr::VRAMCNT_F as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_g(vtype: vram_type::G) {
    unsafe { write_volatile(addr::VRAMCNT_G as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_h(vtype: vram_type::H) {
    unsafe { write_volatile(addr::VRAMCNT_H as *mut u8, vtype as u8); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn map_vram_block_i(vtype: vram_type::I) {
    unsafe { write_volatile(addr::VRAMCNT_I as *mut u8, vtype as u8); }
}
