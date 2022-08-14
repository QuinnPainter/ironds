// todo: could enable atomics using a critical section, like in:
// https://github.com/embassy-rs/atomic-polyfill
// or could provide a custom impl for the critical-section crate:
// https://github.com/embassy-rs/critical-section

// these are macros and not functions, so that they will be inlined for both ARM and THUMB
macro_rules! enable_interrupts_master {
    () => {{
        use core::ptr;
        unsafe { ptr::write_volatile($crate::addr::IME as *mut u32, 1); }
    }}
}
pub(crate) use enable_interrupts_master;

macro_rules! disable_interrupts_master {
    () => {{
        use core::ptr;
        unsafe { ptr::write_volatile($crate::addr::IME as *mut u32, 0); }
    }}
}
pub(crate) use disable_interrupts_master;

macro_rules! is_ime_enabled {
    () => {{
        use core::ptr;
        unsafe { ((ptr::read_volatile($crate::addr::IME as *mut u32) & 1) == 1) }
    }}
}
pub(crate) use is_ime_enabled;

macro_rules! critical_section {
    ($code:block) => {{
        let e = $crate::interrupt::is_ime_enabled!();
        $crate::interrupt::disable_interrupts_master!();
        { $code }
        // only re-enable interrupts if they were enabled before this
        if e { $crate::interrupt::enable_interrupts_master!(); }
    }}
}
pub(crate) use critical_section;
