use super::GfxEngine;
use crate::mmio;
use bitfield_struct::bitfield;
use core::ptr::{read_volatile, write_volatile};

// https://problemkaputt.de/gbatek.htm#lcdobjoverview
// https://problemkaputt.de/gbatek.htm#dsvideoobjs

const AFFINE_FLAG: u64 = 1 << 8;
pub const DISABLED_SPRITE: Sprite = Sprite::NormalSprite(NormalSprite::new().with_disable(true));

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
pub struct NormalSprite {
    pub y: u8,
    _p: bool, // normal sprite indicator (must be 0)
    pub disable: bool,
    #[bits(2)]
    pub mode: u8, //enum
    pub mosaic: bool,
    pub palette_type: bool, //enum
    #[bits(2)]
    pub shape: u8, // enum
    #[bits(9)]
    pub x: u16,
    #[bits(3)]
    _p: u8,
    pub h_flip: bool,
    pub v_flip: bool,
    #[bits(2)]
    pub size: u8,
    #[bits(10)]
    pub tile: u16,
    #[bits(2)]
    pub priority: u8,
    #[bits(4)]
    pub palette: u8,
    _p: u16,
}

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
pub struct AffineSprite {
    pub y: u8,
    _p: bool, // affine sprite indicator (must be 1)
    pub double_size: bool,
    #[bits(2)]
    pub mode: u8, //enum
    pub mosaic: bool,
    pub palette_type: bool, //enum
    #[bits(2)]
    pub shape: u8, // enum
    #[bits(9)]
    pub x: u16,
    #[bits(5)]
    pub affine_param: u8,
    #[bits(2)]
    pub size: u8,
    #[bits(10)]
    pub tile: u16,
    #[bits(2)]
    pub priority: u8,
    #[bits(4)]
    pub palette: u8,
    _p: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sprite {
    NormalSprite(NormalSprite),
    AffineSprite(AffineSprite),
}

#[inline]
pub fn set_sprite(engine: GfxEngine, index: u8, sprite: Sprite) {
    let oam_addr = get_oam_addr(engine, index);

    let obj_data = match sprite {
        Sprite::NormalSprite(s) => u64::from(s) & !AFFINE_FLAG,
        Sprite::AffineSprite(s) => u64::from(s) | AFFINE_FLAG,
    };
    unsafe {
        // Writes OBJ Attributes 0 and 1
        write_volatile(oam_addr as *mut u32, obj_data as u32);
        // Writes OBJ Attribute 2
        write_volatile((oam_addr + 4) as *mut u16, (obj_data >> 32) as u16);
    }
}

#[inline]
pub fn get_sprite(engine: GfxEngine, index: u8) -> Sprite {
    let oam_addr = get_oam_addr(engine, index);

    let mut obj_data: u64;
    unsafe {
        // Reads OBJ Attributes 0 and 1
        obj_data = read_volatile(oam_addr as *const u32) as u64;
        // Reads OBJ Attribute 2
        obj_data |= (read_volatile((oam_addr + 4) as *const u16) as u64) << 32;
    }
    if obj_data & AFFINE_FLAG > 0 {
        Sprite::AffineSprite(AffineSprite::from(obj_data))
    } else {
        Sprite::NormalSprite(NormalSprite::from(obj_data))
    }
}

#[inline(always)]
const fn get_oam_addr(engine: GfxEngine, index: u8) -> usize {
    debug_assert!(index <= 127, "sprite index must be from 0 to 127");

    let oam_addr = match engine {
        GfxEngine::MAIN => mmio::OAM_BASE_MAIN,
        GfxEngine::SUB => mmio::OAM_BASE_SUB,
    };
    oam_addr + (index * 8) as usize // 8 bytes of stride between entries
}
