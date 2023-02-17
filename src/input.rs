//! Module for button input and touchscreen.
#![allow(unused_imports)]

use core::ptr;
use bitflags::bitflags;
use crate::{shared, mmio};

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Buttons: u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const SELECT = 1 << 2;
        const START = 1 << 3;
        const RIGHT = 1 << 4;
        const LEFT = 1 << 5;
        const UP = 1 << 6;
        const DOWN = 1 << 7;
        const R = 1 << 8;
        const L = 1 << 9;
        // 10-15 = Not used
        const X = 1 << 16;
        const Y = 1 << 17;
        // 18 = Not used
        const DEBUG = 1 << 19; // internal contact point, not normally pressable
        // 20, 21 = Not used
        const PEN = 1 << 22; // Pressed whenever the touch screen is touched.
        const HINGE = 1 << 23; // Pressed when the console is closed.
        // 24 - 32 = Not used
    }
}

/// Reads the button state, and updates the shared button state.
/// 
/// Only usable on ARM7.
#[cfg(feature = "arm7")]
pub fn scan_keys() {
    let keys: u32 = ((mmio::EXTKEYIN.read() as u32) << 16) | mmio::KEYINPUT.read() as u32;
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!(shared::SHARED_DATA.buttons.0.bits), !keys);
    }
}

pub fn read_keys() -> Buttons {
    unsafe {
        Buttons::from_bits_retain(ptr::read_volatile(ptr::addr_of!(shared::SHARED_DATA.buttons.0.bits)))
    }
}
