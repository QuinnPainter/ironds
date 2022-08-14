pub mod console;
use core::ptr::{read_volatile, write_volatile};
use bitflags::bitflags;
use voladdress::*;
use crate::addr;

#[cfg(feature = "arm9")]
const POWCNT1: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(addr::POWCNT1) };

// Used with power_on and power_off
bitflags! {
    pub struct GfxPwr: u32 {
        const A_2D = 1 << 1;
        const B_2D = 1 << 9;
        const RENDER_3D = 1 << 2;
        const GEOMETRY_3D = 1 << 3;
        const ALL_2D = Self::A_2D.bits | Self::B_2D.bits;
        const ALL = Self::ALL_2D.bits | Self::RENDER_3D.bits | Self::GEOMETRY_3D.bits; 
    }
}

bitflags! {
    pub struct EngineAPos: u32 {
        const TOP = 1 << 15;
        const BOTTOM = 0;
    }
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
pub fn set_engine_display(pos: EngineAPos) {
    POWCNT1.write((POWCNT1.read() & !EngineAPos::TOP.bits) | pos.bits);
}
