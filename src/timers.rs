use core::ptr;
use crate::addr;

// DS timers are the same as GBA, just incrementing at 33 MHz
// https://problemkaputt.de/gbatek.htm#gbatimers

const BASE_TIMER_ADDR: usize = addr::TM0CNT_L;

pub const PRESCALER_1: u16 = 0;
pub const PRESCALER_64: u16 = 1;
pub const PRESCALER_256: u16 = 2;
pub const PRESCALER_1024: u16 = 3;
pub const COUNT_UP_OFF: u16 = 0;
pub const COUNT_UP_ON: u16 = 0x4;
pub const IRQ_DISABLE: u16 = 0;
pub const IRQ_ENABLE: u16 = 0x40;
pub const TIMER_STOP: u16 = 0;
pub const TIMER_START: u16 = 0x80;

// Use to measure the time taken by code.
// Uses 2 cascading timers for a 32 bit resolution.
// The input is the index of the first timer. (0-2)
// this should maybe be an unsafe fn, since timer_index > 2 could break stuff?
pub fn start_program_timer(timer_index: u32) {
    let first_timer_addr = BASE_TIMER_ADDR + (timer_index * 4) as usize;
    unsafe {
        // stop both timers, and set reload value to 0
        ptr::write_volatile((first_timer_addr + 0) as *mut u32, 0);
        ptr::write_volatile((first_timer_addr + 4) as *mut u32, 0);
        // start first timer
        ptr::write_volatile((first_timer_addr + 2) as *mut u16, PRESCALER_1 | COUNT_UP_OFF | IRQ_DISABLE | TIMER_START);
        // start second timer
        ptr::write_volatile((first_timer_addr + 6) as *mut u16, PRESCALER_1 | COUNT_UP_ON | IRQ_DISABLE | TIMER_START);
    }
}

pub fn stop_program_timer(timer_index: u32) -> u32 {
    let first_timer_addr = BASE_TIMER_ADDR + (timer_index * 4) as usize;
    unsafe {
        let low: u16 = ptr::read_volatile((first_timer_addr + 0) as *mut u16);
        let high: u16 = ptr::read_volatile((first_timer_addr + 4) as *mut u16);
        // stop both timers
        ptr::write_volatile((first_timer_addr + 0) as *mut u32, 0);
        ptr::write_volatile((first_timer_addr + 4) as *mut u32, 0);
        return ((high as u32) << 16) | (low as u32);
    }
}
