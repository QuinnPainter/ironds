//! Module that provides access to the hardware interrupts.

// todo: could enable atomics using a critical section, like in:
// https://github.com/embassy-rs/atomic-polyfill
// or could provide a custom impl for the critical-section crate:
// https://github.com/embassy-rs/critical-section
use core::ptr::{read_volatile, write_volatile};
use core::arch::global_asm;
use bitflags::bitflags;
use crate::mmio;

global_asm! {
    include_str!("irq_handler.s"),
    options(raw)
}

/// Enables the Interrupt Master Enable, allowing interrupts to run.
#[inline(always)]
pub fn enable_ime() {
    unsafe { write_volatile(mmio::IME as *mut u32, 1); }
}

/// Disables the Interrupt Master Enable, preventing all interrupts from running.
#[inline(always)]
pub fn disable_ime() {
    unsafe { write_volatile(mmio::IME as *mut u32, 0); }
}

/// Checks if the Interrupt Master Enable is currently enabled.
/// 
/// Returns `false` if interrupts are disabled, `true` if they are enabled.
#[inline(always)]
pub fn read_ime() -> bool {
    unsafe { (read_volatile(mmio::IME as *mut u32) & 1) == 1 }
}

/// Prevents interrupts from occuring during a certain block of code.
/// 
/// Use this if you have some timing critical code, or if you need to manipulate some static data
/// and need it to be thread-safe.
/// # Examples
/// 
/// ```
/// critical_section!({
///     // important code here
/// });
/// ```
pub macro critical_section($code:block) {
    let e = read_ime();
    disable_ime();
    { $code }
    // only re-enable interrupts if they were enabled before this
    if e { enable_ime(); }
}

#[no_mangle]
#[cfg_attr(feature = "arm9", link_section = ".itcm.irq_handler_ptr")]
#[cfg_attr(feature = "arm7", link_section = ".iwram.irq_handler_ptr")]
static mut USER_IRQ_HANDLER: Option<extern "C" fn(IRQFlags)> = None;

pub enum IRQType {
    Vblank = 0,
    Hblank = 1,
    Vcount = 2,
    Timer0 = 3,
    Timer1 = 4,
    Timer2 = 5,
    Timer3 = 6,
    Serial = 7, // ARM7 only
    DMA0 = 8,
    DMA1 = 9,
    DMA2 = 10,
    DMA3 = 11,
    Keypad = 12,
    Slot2 = 13, // Connected to GBA slot
    // 14 and 15 unused
    IPCSync = 16,
    IPCSendFifoEmpty = 17,
    IPCRecvFifoNotEmpty = 18,
    CartTransfer = 19,
    // An IRQ line connected to the cartridge. Usually just grounded (except in Pokemon Typing Adventure?)
    // Could maybe be used to detect cartridge removal.
    CartIREQ = 20,
    GeometryFIFO = 21, // ARM9 only
    Lid = 22, // ARM7 only
    SPI = 23, // ARM7 only
    Wifi = 24, // ARM7 only
}

bitflags! {
    #[repr(transparent)]
    pub struct IRQFlags: u32 {
        const VBLANK = 1 << IRQType::Vblank as u32;
        const HBLANK = 1 << IRQType::Hblank as u32;
        const VCOUNT = 1 << IRQType::Vcount as u32;
        const TIMER0 = 1 << IRQType::Timer0 as u32;
        const TIMER1 = 1 << IRQType::Timer1 as u32;
        const TIMER2 = 1 << IRQType::Timer2 as u32;
        const TIMER3 = 1 << IRQType::Timer3 as u32;
        const SERIAL = 1 << IRQType::Serial as u32;
        const DMA0 = 1 << IRQType::DMA0 as u32;
        const DMA1 = 1 << IRQType::DMA1 as u32;
        const DMA2 = 1 << IRQType::DMA2 as u32;
        const DMA3 = 1 << IRQType::DMA3 as u32;
        const KEYPAD = 1 << IRQType::Keypad as u32;
        const SLOT2 = 1 << IRQType::Slot2 as u32;
        const IPC_SYNC = 1 << IRQType::IPCSync as u32;
        const IPC_SEND_FIFO_EMPTY = 1 << IRQType::IPCSendFifoEmpty as u32;
        const IPC_RECV_FIFO_NOT_EMPTY = 1 << IRQType::IPCRecvFifoNotEmpty as u32;
        const CART_TRANSFER = 1 << IRQType::CartTransfer as u32;
        const CART_IREQ = 1 << IRQType::CartIREQ as u32;
        const GEOMETRY_FIFO = 1 << IRQType::GeometryFIFO as u32;
        const LID = 1 << IRQType::Lid as u32;
        const SPI = 1 << IRQType::SPI as u32;
        const WIFI = 1 << IRQType::Wifi as u32;
    }
}

#[inline(always)]
pub fn irq_set_handler(f: Option<extern "C" fn(IRQFlags)>) {
    unsafe { USER_IRQ_HANDLER = f; }
}

pub fn irq_enable(flags: IRQFlags) {
    critical_section!({
        unsafe { write_volatile(mmio::IE as *mut u32, read_volatile(mmio::IE as *mut u32) | flags.bits()); }
        // todo: are these dispstat flags shared between ARM9/ARM7?
        // probably are, so shouldn't enable / disable them here
        // should just leave them on all the time?
        if flags & IRQFlags::VBLANK == IRQFlags::VBLANK {
            mmio::DISPSTAT.write(mmio::DISPSTAT.read() | (1 << 3));
        }
        if flags & IRQFlags::HBLANK == IRQFlags::HBLANK {
            mmio::DISPSTAT.write(mmio::DISPSTAT.read() | (1 << 4));
        }
        if flags & IRQFlags::VCOUNT == IRQFlags::VCOUNT {
            mmio::DISPSTAT.write(mmio::DISPSTAT.read() | (1 << 5));
        }
        // todo: also enable IPCSync
        // also make sure cpsr irq thing is enabled??
    });
}

pub fn irq_disable(flags: IRQFlags) {
    critical_section!({
        unsafe { write_volatile(mmio::IE as *mut u32, read_volatile(mmio::IE as *mut u32) & !flags.bits()); }
        if flags & IRQFlags::VBLANK == IRQFlags::VBLANK {
            mmio::DISPSTAT.write(mmio::DISPSTAT.read() & !(1 << 3));
        }
        if flags & IRQFlags::HBLANK == IRQFlags::HBLANK {
            mmio::DISPSTAT.write(mmio::DISPSTAT.read() & !(1 << 4));
        }
        if flags & IRQFlags::VCOUNT == IRQFlags::VCOUNT {
            mmio::DISPSTAT.write(mmio::DISPSTAT.read() & !(1 << 5));
        }
        // todo: also disable IPCSync
    });
}
