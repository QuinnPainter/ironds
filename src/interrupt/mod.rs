// todo: could enable atomics using a critical section, like in:
// https://github.com/embassy-rs/atomic-polyfill
// or could provide a custom impl for the critical-section crate:
// https://github.com/embassy-rs/critical-section
use core::ptr;
use crate::addr;

// these are macros and not functions, so that they will be inlined for both ARM and THUMB
pub macro enable_interrupts_master() {
    unsafe { ptr::write_volatile(addr::IME as *mut u32, 1); }
}

pub macro disable_interrupts_master() {
    unsafe { ptr::write_volatile(addr::IME as *mut u32, 0); }
}

pub macro is_ime_enabled() {
    unsafe { ((ptr::read_volatile(addr::IME as *mut u32) & 1) == 1) }
}

pub macro critical_section($code:block) {
    let e = crate::interrupt::is_ime_enabled!();
    crate::interrupt::disable_interrupts_master!();
    { $code }
    // only re-enable interrupts if they were enabled before this
    if e { crate::interrupt::enable_interrupts_master!(); }
}

// Run whenever an IRQ is triggered. Should not be called by user code.
#[instruction_set(arm::a32)]
pub fn irq_handler() {}
