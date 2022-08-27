#![no_std]
//#![feature(default_alloc_error_handler)]
#![allow(unused_macros, unused_imports)]
#![feature(decl_macro)]
#![feature(isa_attribute)]

#[cfg(all(feature = "arm9", feature = "arm7"))]
compile_error!("feature \"arm9\" and feature \"arm7\" cannot be enabled at the same time");

//extern crate alloc;
//use alloc::string::String;
//use core::fmt;
//use linked_list_allocator::Heap;

//#[global_allocator]
//static ALLOCATOR: Heap = Heap::empty();

//use interrupt::critical_section;
use interrupt::disable_interrupts_master;
//use crate::debug::nocash;

pub mod runtime;
pub mod nocash;
pub mod interrupt;
pub mod display;
pub mod timers;
pub mod addr;

extern "C" {
    static __heap_start: *mut u8;
    static __heap_size: usize;
}

// this function is called from init.s, before main.
// interrupts are disabled at this point, so no need to worry about thread-safety
#[no_mangle]
extern "C" fn lib_init() {
    // turn on all graphics engines
    display::power_on(display::GfxPwr::ALL);
    // set brightness to default level
    display::set_brightness(display::GfxEngine::MAIN, 0);
    display::set_brightness(display::GfxEngine::SUB, 0);
    //critical_section!({ nocash::print("stuff"); });
    //unsafe {
    //    ALLOCATOR.lock().init(__heap_start, __heap_size);
    //}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    disable_interrupts_master!();
    //let mut output = String::new();
	//fmt::write(&mut output, format_args!("{}", _info));
    loop {}
}
