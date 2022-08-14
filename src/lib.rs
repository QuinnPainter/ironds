#![no_std]
//#![feature(default_alloc_error_handler)]
#![allow(unused_macros, unused_imports)]
#![feature(decl_macro)]

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
pub mod timers;

extern "C" {
    static __heap_start: *mut u8;
    static __heap_size: usize;
}

#[no_mangle]
extern "C" fn lib_init() {
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
