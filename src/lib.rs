#![no_std]
#![feature(decl_macro)]
#![feature(isa_attribute)]
#![feature(alloc_error_handler)]

#[cfg(all(feature = "arm9", feature = "arm7"))]
compile_error!("Feature \"arm9\" and feature \"arm7\" cannot be enabled at the same time");
#[cfg(not(any(feature = "arm9", feature = "arm7")))]
compile_error!("Either feature \"arm9\" or \"arm7\" must be enabled");

extern crate alloc;
use alloc::string::String;
use core::fmt::Write;

#[global_allocator]
#[cfg_attr(feature = "arm9", link_section = ".dtcm.alloc")]
#[cfg_attr(feature = "arm7", link_section = ".iwram.alloc")]
static mut ALLOCATOR: allocator::ACSLAlloc = allocator::ACSLAlloc::new();

pub mod runtime;
pub mod nocash;
pub mod interrupt;
pub mod allocator;
#[cfg(feature = "arm9")]
pub mod display;
pub mod timers;
pub mod addr;
pub mod agbabi;
pub mod syscall;

// Accessing variables from the linkerscript is weird.
// https://stackoverflow.com/questions/72820626/how-to-access-a-variable-from-linker-script-in-rust-code?noredirect=1&lq=1
#[inline(always)]
fn heap_start() -> *mut u8 {
    extern "C" { static __heap_start: *mut u8; }
    unsafe { &__heap_start as *const _ as *mut u8 }
}

#[inline(always)]
fn heap_size() -> usize {
    extern "C" { static __heap_size: usize; }
    unsafe { &__heap_size as *const _ as usize }
}

/*#[inline(always)]
fn irq_vec() -> *mut usize {
    extern "C" { static __irq_vec: *mut usize; }
    unsafe { &__irq_vec as *const _ as *mut usize }
}

#[inline(always)]
fn irq_flags() -> *mut u32 {
    extern "C" { static __irq_flags: *mut u32; }
    unsafe { &__irq_flags as *const _ as *mut u32 }
}*/

// this function is called from init.s, before main.
// interrupts are disabled at this point, so no need to worry about thread-safety
#[no_mangle]
extern "C" fn lib_init() {
    #[cfg(feature = "arm9")]
    {
        // turn on all graphics engines
        display::power_on(display::GfxPwr::ALL);
        // set brightness to default level
        display::set_brightness(display::GfxEngine::MAIN, 0);
        display::set_brightness(display::GfxEngine::SUB, 0);
    }
    //unsafe { core::ptr::write(irq_vec(), interrupt::irq_handler as usize); }
    unsafe { ALLOCATOR.init(heap_start(), heap_size()); }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // concat! doesn't like const strings, this works as a workaround
    macro_rules! ERR_HEADER { () => { "      ---- ARM9 PANIC ----\n\n" }; }

    interrupt::disable_interrupts_master!();
    let mut output: String = String::new();
    let printed_output: &str;
    // Reserve enough chars to fill the screen
    if output.try_reserve_exact(32 * 24).is_err() {
        printed_output = concat!(ERR_HEADER!(), "Allocation failed: Out of memory");
    }
    else {
        printed_output = match write!(&mut output, "{}", info) {
            Ok(_) => { output.insert_str(0, ERR_HEADER!()); output.as_str() },
            Err(_) => concat!(ERR_HEADER!(), "Error formatting panic message.\nHow did this happen?"),
        };
    }
    #[cfg(feature = "arm9")]
    {
        display::console::init_default();
        display::console::print(printed_output);
        // arm9 panic should send message to halt arm7?
    }
    // todo: arm7 panic should send message to arm9 to display message
    loop {}
}

#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size())
}

// why are these needed??? rust shouldn't be calling C++ exception unwinding code
// can be fixed by enabling lto, maybe? https://blog.bokuweb.me/entry/2020/04/14/101202
#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
pub fn __aeabi_unwind_cpp_pr1() {}
