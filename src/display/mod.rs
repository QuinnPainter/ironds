pub mod console;
pub mod obj;
mod vram;
pub use vram::*;

use core::ptr::{read_volatile, write_volatile};
use bitflags::bitflags;
use bitfield_struct::bitfield;
use voladdress::*;
use crate::mmio;

#[cfg(feature = "arm9")]
const POWCNT1: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(mmio::POWCNT1) };

// Used with power_on and power_off
bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GfxPwr: u32 {
        const MAIN_2D = 1 << 1;
        const SUB_2D = 1 << 9;
        const RENDER_3D = 1 << 2;
        const GEOMETRY_3D = 1 << 3;
        const ALL_2D = Self::MAIN_2D.bits() | Self::SUB_2D.bits();
        const ALL = Self::ALL_2D.bits() | Self::RENDER_3D.bits() | Self::GEOMETRY_3D.bits(); 
    }
}

#[bitfield(u32)]
pub struct DisplayControlMain {
    #[bits(3)]
    pub bg_mode: u8, // enum
    pub bg0_3d: bool,
    pub tile_obj_mapping: bool, // enum
    pub bm_obj_2d_dim: bool, // enum
    pub bm_obj_mapping: bool, // enum
    pub forced_blank: bool,
    pub display_bg0: bool,
    pub display_bg1: bool,
    pub display_bg2: bool,
    pub display_bg3: bool,
    pub display_obj: bool,
    pub display_win0: bool,
    pub display_win1: bool,
    pub display_obj_win: bool,
    #[bits(2)]
    pub display_mode: u8, // enum
    #[bits(2)]
    pub vram_display_block: u8, // enum
    #[bits(2)]
    pub tile_obj_1d_bound: u8,
    #[bits(1)]
    pub bm_obj_1d_bound: u8,
    pub obj_during_hblank: bool,
    #[bits(3)]
    pub master_tiledata_base: u8,
    #[bits(3)]
    pub master_tilemap_base: u8,
    pub bg_ext_pal_enabled: bool,
    pub obj_ext_pal_enabled: bool,
}

#[bitfield(u32)]
pub struct DisplayControlSub {
    #[bits(3)]
    pub bg_mode: u8, // enum (different)
    _p: bool,
    pub tile_obj_mapping: bool, // enum
    pub bm_obj_2d_dim: bool, // enum
    pub bm_obj_mapping: bool, // enum
    pub forced_blank: bool,
    pub display_bg0: bool,
    pub display_bg1: bool,
    pub display_bg2: bool,
    pub display_bg3: bool,
    pub display_obj: bool,
    pub display_win0: bool,
    pub display_win1: bool,
    pub display_obj_win: bool,
    #[bits(2)]
    pub display_mode: u8, // enum (different)
    #[bits(2)]
    _p: u8,
    #[bits(2)]
    pub tile_obj_1d_bound: u8,
    _p: bool,
    pub obj_during_hblank: bool,
    #[bits(6)]
    _p: u8,
    pub bg_ext_pal_enabled: bool,
    pub obj_ext_pal_enabled: bool,
}

#[bitfield(u16)]
pub struct BackgroundControl {
    #[bits(2)]
    pub priority: u8, // lower = higher priority
    #[bits(4)]
    pub tiledata_base: u8,
    pub mosaic_enabled: bool,
    #[bits(1)]
    pub palette_setting: u8, // enum
    #[bits(5)]
    pub tilemap_base: u8,
    #[bits(1)]
    pub bit13: u8, // BG0/BG1 = Ext Palette Slot. BG2/BG3 = Display Area Overflow (0=Transparent, 1=Wraparound)
    #[bits(2)]
    pub screen_size: u8,
}

pub enum MainEnginePos {
    TOP = 1 << 15,
    BOTTOM = 0,
}

pub enum GfxEngine {
    MAIN = 0,
    SUB = 0x1000,
}

/// Converts a standard hexcode (0xRRGGBB) to the 15-bit palette colour format.
#[inline(always)]
pub const fn rgb15(x: u32) -> u16 {
    (((x & 0xF80000) >> 19) | ((x & 0x00F800) >> 6) | ((x & 0x0000F8) << 7)) as u16
}

/// Turns the specified graphics engines on (using POWCNT1).
#[cfg(feature = "arm9")]
pub fn power_on(pwrflags: GfxPwr) {
    POWCNT1.write(POWCNT1.read() | pwrflags.bits());
}

/// Turns the specified graphics engines off (using POWCNT1).
#[cfg(feature = "arm9")]
pub fn power_off(pwrflags: GfxPwr) {
    POWCNT1.write(POWCNT1.read() & !pwrflags.bits());
}

/// Sets which graphics engine corresponds with which display (top or bottom).
#[cfg(feature = "arm9")]
pub fn set_engine_lcd(pos: MainEnginePos) {
    POWCNT1.write((POWCNT1.read() & !(MainEnginePos::TOP as u32)) | pos as u32);
}

/// Sets the master brightness for one of the graphics engines.
/// 
/// Brightness value can be from -16 to 16 (0 is default)  
/// This doesn't set the backlight brightness, only applies a "colour correction"  
/// -16 is pure black, 16 is pure white
#[cfg(feature = "arm9")]
pub fn set_brightness(engine: GfxEngine, mut brightness: i32) {
    let master_bright = (mmio::MASTER_BRIGHT_MAIN | engine as usize) as *mut u32;
    let mut mode: u32 = 1 << 14; // up
    if brightness < 0 {
        brightness = -brightness; // adjust to positive
        mode = 2 << 14; // down
    }
    if brightness > 16 { brightness = 16; }

    unsafe { write_volatile(master_bright, mode | (brightness as u32)); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn set_main_display_control(c: DisplayControlMain) {
    unsafe { write_volatile(mmio::DISPCNT_MAIN as *mut u32, u32::from(c)); }
}

#[must_use]
#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_main_display_control() -> DisplayControlMain {
    unsafe { DisplayControlMain::from(read_volatile(mmio::DISPCNT_MAIN as *mut u32)) }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn set_sub_display_control(c: DisplayControlSub) {
    unsafe { write_volatile(mmio::DISPCNT_SUB as *mut u32, u32::from(c)); }
}

#[must_use]
#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_sub_display_control() -> DisplayControlSub {
    unsafe { DisplayControlSub::from(read_volatile(mmio::DISPCNT_SUB as *mut u32)) }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn set_main_bg_control(bg: usize, c: BackgroundControl) {
    unsafe { write_volatile((mmio::BG0CNT_MAIN + ((bg & 0x3) * 2)) as *mut u16, u16::from(c)); }
}

#[must_use]
#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_main_bg_control(bg: usize) -> BackgroundControl {
    unsafe { BackgroundControl::from(read_volatile((mmio::BG0CNT_MAIN + ((bg & 0x3) * 2)) as *mut u16)) }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn set_sub_bg_control(bg: usize, c: BackgroundControl) {
    unsafe { write_volatile((mmio::BG0CNT_SUB + ((bg & 0x3) * 2)) as *mut u16, u16::from(c)); }
}

#[must_use]
#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_sub_bg_control(bg: usize) -> BackgroundControl {
    unsafe { BackgroundControl::from(read_volatile((mmio::BG0CNT_SUB + ((bg & 0x3) * 2)) as *mut u16)) }
}

/// Set the screen line that the VCounter is triggered for.
/// 
/// Valid values are from 0 to 262.
/// 0 is the top of the screen, 191 is the bottom.
/// 192 to 262 are during VBlank.
#[inline]
pub fn set_vcount_trigger(line: u16) {
    debug_assert!(line < 263, "vcount trigger must be from 0 to 262 (was: {line})");
    mmio::DISPSTAT.apply(|x| *x = (*x & 0x007F) | (line << 8) | ((line >> 1) & 0x80));
}
