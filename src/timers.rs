//! Module for utilising the hardware timers.
#![allow(dead_code)]

use core::ptr;
use crate::addr;

// DS timers are the same as GBA, just incrementing at 33 MHz
// https://problemkaputt.de/gbatek.htm#gbatimers

const BASE_TIMER_ADDR: usize = addr::TM0CNT_L;

const PRESCALER_1: u16 = 0;
const PRESCALER_64: u16 = 1;
const PRESCALER_256: u16 = 2;
const PRESCALER_1024: u16 = 3;
const COUNT_UP_OFF: u16 = 0;
const COUNT_UP_ON: u16 = 0x4;
const IRQ_DISABLE: u16 = 0;
const IRQ_ENABLE: u16 = 0x40;
const TIMER_STOP: u16 = 0;
const TIMER_START: u16 = 0x80;

/// Starts a profiler timer, used to measure code execution time.
/// 
/// Uses 2 cascading hardware timers for a 32 bit resolution.
/// The input is the index of the first timer (0-2).
pub fn start_profiler_timer(timer_index: u32) {
    debug_assert!(timer_index <= 2, "invalid timer index for start_profiler_timer (must be 0 to 2)");
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

/// Ends a profiler timer, and returns the time taken.
/// 
/// Make sure to pass in the same timer index used for [`start_profiler_timer`].
pub fn end_profiler_timer(timer_index: u32) -> u32 {
    debug_assert!(timer_index <= 2, "invalid timer index for end_profiler_timer (must be 0 to 2)");
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
