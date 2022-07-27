#![no_std]
//#![feature(default_alloc_error_handler)]
#![allow(unused_macros, unused_imports)]

//extern crate alloc;
//use alloc::string::String;
//use core::fmt;
//use linked_list_allocator::Heap;

//#[global_allocator]
//static ALLOCATOR: Heap = Heap::empty();

//use interrupt::critical_section;
//use crate::debug::nocash;

pub mod runtime;
pub mod debugging;
pub mod interrupt;

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
    //let mut output = String::new();
	//fmt::write(&mut output, format_args!("{}", _info));
    loop {}
}
