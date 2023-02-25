use core::ptr::write_volatile;
use bitfield_struct::bitfield;
use super::GfxEngine;
use crate::mmio;

// https://problemkaputt.de/gbatek.htm#lcdobjoverview
// https://problemkaputt.de/gbatek.htm#dsvideoobjs

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
    pub size: u8, // enum?
    #[bits(10)]
    pub tile: u16,
    #[bits(2)]
    pub priority: u8,
    #[bits(4)]
    pub palette: u8,
    _p: u16
}

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
pub struct AffineSprite {
    pub y: u8,
    #[bits(56)]
    _p: u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sprite {
    NormalSprite(NormalSprite),
    AffineSprite(AffineSprite),
}

#[inline]
pub fn set_sprite(engine: GfxEngine, index: u8, sprite: Sprite) {
    debug_assert!(index <= 127, "sprite index must be from 0 to 127");
    const AFFINE_FLAG: u64 = 1 << 8;

    let oam_addr = match engine {
        GfxEngine::MAIN => mmio::OAM_BASE_MAIN,
        GfxEngine::SUB => mmio::OAM_BASE_SUB
    };
    let oam_addr = oam_addr + (index * 8) as usize; // 8 bytes of stride between entries

    let obj_data = match sprite {
        Sprite::NormalSprite(s) => u64::from(s) & !AFFINE_FLAG,
        Sprite::AffineSprite(s) => u64::from(s) | AFFINE_FLAG
    };
    unsafe {
        // writes OBJ Attributes 0 and 1
        write_volatile(oam_addr as *mut u32, obj_data as u32);
        // Writes OBJ Attribute 2
        write_volatile((oam_addr + 4) as *mut u16, (obj_data >> 32) as u16);
    }
}
