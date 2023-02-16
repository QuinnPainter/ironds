//! Module that provides a simple way to draw text to the screen.
//! 
//! This module is very limited, and should only really be used for debug purposes.

use crate::{display, mmio};
use core::mem::size_of;

static DEFAULT_FONT: &[u8; 4096] = include_bytes!("../../gfx/font.img.bin");
const DEFAULT_PALETTE: [u16; 2] = [display::rgb15(0x000000), display::rgb15(0xFFFFFF)];
const TILES_PER_LINE: usize = 32;
const BYTES_PER_LINE: usize = TILES_PER_LINE * 2;

static mut CURSOR_X: u8 = 0;
static mut CURSOR_Y: u8 = 0;

/// Initialises the text console.
/// 
/// You must run this function before attempting to `print`.  
/// Uses the Sub graphics engine, and loads into VRAM block H.
pub fn init_default() {
    // Make sure the 2D graphics engines are turned on
    display::power_on(display::GfxPwr::ALL_2D);

    // set brightness to default level
    display::set_brightness(display::GfxEngine::SUB, 0);

    display::map_vram_block_h(display::vram_type::H::SUB_BG);

    display::set_sub_display_control(display::DisplayControlSub::new()
        .with_bg_mode(0)
        .with_display_bg0(true)
        .with_display_bg1(false)
        .with_display_bg2(false)
        .with_display_bg3(false)
        .with_display_obj(false)
        .with_display_win0(false)
        .with_display_win1(false)
        .with_display_mode(1)
        .with_bg_ext_pal_enabled(false));

    display::set_sub_bg_control(0, display::BackgroundControl::new()
        .with_priority(0)
        .with_tiledata_base(0)
        .with_mosaic_enabled(false)
        .with_palette_setting(0)
        .with_tilemap_base(4) // 8K offset
        .with_screen_size(0));

    unsafe { core::ptr::write_volatile(mmio::BG0XOFS_MAIN as *mut u16, 0); }
    unsafe { core::ptr::write_volatile(mmio::BG0YOFS_MAIN as *mut u16, 0); }

    unsafe {
        // fill tilemap with space characters
        core::ptr::write_bytes((0x6200000 + 8192) as *mut u16, 32, 32 * 24);

        // load font into sub-bg VRAM
        // vram doesn't support 8 bit loads, must load as 16 bit
        core::ptr::copy_nonoverlapping(DEFAULT_FONT.as_ptr() as *const u16,
            0x06200000 as *mut u16, DEFAULT_FONT.len() / 2);

        // load palette into sub-bg palette RAM
        core::ptr::copy_nonoverlapping(DEFAULT_PALETTE.as_ptr(),
            0x05000400 as *mut u16, DEFAULT_PALETTE.len() * size_of::<u16>());
    }

    set_cursor_pos(0, 0);
}

/// Prints some text to the screen.
/// 
/// When text reaches the right edge of the screen, it will wrap around to the left side and move to the next line.  
/// Make sure you have initialised the console before running this function.
pub fn print(txt: &str) {
    for mut b in txt.bytes() {
        if b == b'\n' {
            unsafe { CURSOR_Y += 1; CURSOR_X = 0; }
            continue;
        }
        // Control chars / extended UTF8 chars get mapped to 0 ("tofu" character)
        if b < 32 || b > 126 {
            b = 0;
        }
        unsafe { core::ptr::write((0x6200000 + 8192
            + (CURSOR_Y as usize * BYTES_PER_LINE) 
            + (CURSOR_X as usize * 2)) as *mut u16, b as u16); }
        unsafe {
            CURSOR_X += 1;
            if CURSOR_X >= 32 {
                CURSOR_X = 0;
                CURSOR_Y += 1;
                // if print goes off the bottom, all further prints should be ignored
                if CURSOR_Y >= 32 { return; }
            }
        }
    }
}

/// Returns the current position of the text cursor. (0, 0) is the top left.
#[must_use]
#[inline(always)]
pub fn get_cursor_pos() -> (u8, u8) {
    unsafe { (CURSOR_X, CURSOR_Y) }
}

/// Sets the current position of the text cursor. (0, 0) is the top left.
#[inline(always)]
pub fn set_cursor_pos(x: u8, y: u8) {
    unsafe { CURSOR_X = x & 31; CURSOR_Y = y & 31; }
}
