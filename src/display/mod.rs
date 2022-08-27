pub mod console;
mod vram;
pub use vram::*;

use core::ptr::{read_volatile, write_volatile};
use bitflags::bitflags;
use modular_bitfield::prelude::*;
use voladdress::*;
use crate::addr;

#[cfg(feature = "arm9")]
const POWCNT1: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(addr::POWCNT1) };

// Used with power_on and power_off
bitflags! {
    pub struct GfxPwr: u32 {
        const MAIN_2D = 1 << 1;
        const SUB_2D = 1 << 9;
        const RENDER_3D = 1 << 2;
        const GEOMETRY_3D = 1 << 3;
        const ALL_2D = Self::MAIN_2D.bits | Self::SUB_2D.bits;
        const ALL = Self::ALL_2D.bits | Self::RENDER_3D.bits | Self::GEOMETRY_3D.bits; 
    }
}

#[bitfield]
#[repr(u32)]
pub struct DisplayControlMain {
    bg_mode: B3, // enum
    bg0_3d: bool,
    tile_obj_mapping: bool, // enum
    bm_obj_2d_dim: bool, // enum
    bm_obj_mapping: bool, // enum
    forced_blank: bool,
    display_bg0: bool,
    display_bg1: bool,
    display_bg2: bool,
    display_bg3: bool,
    display_obj: bool,
    display_win0: bool,
    display_win1: bool,
    display_obj_win: bool,
    display_mode: B2, // enum
    vram_display_block: B2, // enum
    tile_obj_1d_bound: B2,
    bm_obj_1d_bound: B1,
    obj_during_hblank: bool,
    master_tiledata_base: B3,
    master_tilemap_base: B3,
    bg_ext_pal_enabled: bool,
    obj_ext_pal_enabled: bool,
}

#[bitfield]
#[repr(u32)]
pub struct DisplayControlSub {
    bg_mode: B3, // enum (different)
    #[skip] __: bool,
    tile_obj_mapping: bool, // enum
    bm_obj_2d_dim: bool, // enum
    bm_obj_mapping: bool, // enum
    forced_blank: bool,
    display_bg0: bool,
    display_bg1: bool,
    display_bg2: bool,
    display_bg3: bool,
    display_obj: bool,
    display_win0: bool,
    display_win1: bool,
    display_obj_win: bool,
    display_mode: B2, // enum (different)
    #[skip] __: B2,
    tile_obj_1d_bound: B2,
    #[skip] __: B1,
    obj_during_hblank: bool,
    #[skip] __: B6,
    bg_ext_pal_enabled: bool,
    obj_ext_pal_enabled: bool,
}

#[bitfield]
#[repr(u16)]
pub struct BackgroundControl {
    priority: B2, // lower = higher priority
    tiledata_base: B4,
    mosaic_enabled: bool,
    palette_setting: B1, // enum
    tilemap_base: B5,
    bit13: B1, // BG0/BG1 = Ext Palette Slot. BG2/BG3 = Display Area Overflow (0=Transparent, 1=Wraparound)
    screen_size: B2,
}

pub enum MainEnginePos {
    TOP = 1 << 15,
    BOTTOM = 0,
}

pub enum GfxEngine {
    MAIN = 0,
    SUB = 0x1000,
}

// Converts a standard hexcode (0xRRGGBB) to the 15-bit palette colour format
#[inline(always)]
pub const fn rgb15(x: u32) -> u16 {
    (((x & 0xF80000) >> 19) | ((x & 0x00F800) >> 6) | ((x & 0x0000F8) << 7)) as u16
}

// Turns the specified graphics engines on (using POWCNT1)
#[cfg(feature = "arm9")]
pub fn power_on(pwrflags: GfxPwr) {
    POWCNT1.write(POWCNT1.read() | pwrflags.bits);
}

// Turns the specified graphics engines off (using POWCNT1)
#[cfg(feature = "arm9")]
pub fn power_off(pwrflags: GfxPwr) {
    POWCNT1.write(POWCNT1.read() & !pwrflags.bits);
}

// Sets which graphics engine corresponds with which display (top or bottom)
#[cfg(feature = "arm9")]
pub fn set_engine_lcd(pos: MainEnginePos) {
    POWCNT1.write((POWCNT1.read() & !(MainEnginePos::TOP as u32)) | pos as u32);
}

// Set the master brightness for one of the graphics engines.
// Brightness value can be from -16 to 16 (0 is default)
// This doesn't set the backlight brightness, only applies a "colour correction"
// -16 is pure black, 16 is pure white
#[cfg(feature = "arm9")]
pub fn set_brightness(engine: GfxEngine, mut brightness: i32) {
    let master_bright = (addr::MASTER_BRIGHT_MAIN | engine as usize) as *mut u32;
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
    unsafe { write_volatile(addr::DISPCNT_MAIN as *mut u32, u32::from(c)); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_main_display_control() -> DisplayControlMain {
    unsafe { DisplayControlMain::from(read_volatile(addr::DISPCNT_MAIN as *mut u32)) }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn set_sub_display_control(c: DisplayControlSub) {
    unsafe { write_volatile(addr::DISPCNT_SUB as *mut u32, u32::from(c)); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_sub_display_control() -> DisplayControlSub {
    unsafe { DisplayControlSub::from(read_volatile(addr::DISPCNT_SUB as *mut u32)) }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn set_main_bg_control(bg: usize, c: BackgroundControl) {
    unsafe { write_volatile((addr::BG0CNT_MAIN + ((bg & 0x3) * 2)) as *mut u16, u16::from(c)); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_main_bg_control(bg: usize) -> BackgroundControl {
    unsafe { BackgroundControl::from(read_volatile((addr::BG0CNT_MAIN + ((bg & 0x3) * 2)) as *mut u16)) }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn set_sub_bg_control(bg: usize, c: BackgroundControl) {
    unsafe { write_volatile((addr::BG0CNT_SUB + ((bg & 0x3) * 2)) as *mut u16, u16::from(c)); }
}

#[cfg(feature = "arm9")]
#[inline(always)]
pub fn get_sub_bg_control(bg: usize) -> BackgroundControl {
    unsafe { BackgroundControl::from(read_volatile((addr::BG0CNT_SUB + ((bg & 0x3) * 2)) as *mut u16)) }
}
