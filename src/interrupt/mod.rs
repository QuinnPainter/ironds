// todo: could enable atomics using a critical section, like in:
// https://github.com/embassy-rs/atomic-polyfill
// or could provide a custom impl for the critical-section crate:
// https://github.com/embassy-rs/critical-section
use core::ptr::{read_volatile, write_volatile};
use core::arch::global_asm;
use bitflags::bitflags;
use crate::addr;

// these are macros and not functions, so that they will be inlined for both ARM and THUMB
pub macro enable_interrupts_master() {
    unsafe { write_volatile(addr::IME as *mut u32, 1); }
}

pub macro disable_interrupts_master() {
    unsafe { write_volatile(addr::IME as *mut u32, 0); }
}

pub macro is_ime_enabled() {
    unsafe { ((read_volatile(addr::IME as *mut u32) & 1) == 1) }
}

pub macro critical_section($code:block) {
    let e = crate::interrupt::is_ime_enabled!();
    crate::interrupt::disable_interrupts_master!();
    { $code }
    // only re-enable interrupts if they were enabled before this
    if e { crate::interrupt::enable_interrupts_master!(); }
}

#[no_mangle]
#[cfg_attr(feature = "arm9", link_section = ".itcm.irq_table")]
#[cfg_attr(feature = "arm7", link_section = ".iwram.irq_table")]
static mut IRQ_TABLE: [usize; 25] = [0; 25];

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

global_asm! {
    include_str!("irq_handler.s"),
    options(raw)
}

pub fn irq_set_fn(t: IRQType, f: fn()) {
    critical_section!({
        unsafe { IRQ_TABLE[t as usize] = f as usize; }
    });
}

pub fn irq_unset_fn(t: IRQType) {
    critical_section!({
        unsafe { IRQ_TABLE[t as usize] = 0; }
    });
}

pub fn irq_enable(flags: IRQFlags) {
    critical_section!({
        unsafe { write_volatile(addr::IE as *mut u32, read_volatile(addr::IE as *mut u32) | flags.bits()); }
        // todo: also enable interrupt sources (vblank etc)
        // also make sure cpsr irq thing is enabled??
    });
}

pub fn irq_disable(flags: IRQFlags) {
    critical_section!({
        unsafe { write_volatile(addr::IE as *mut u32, read_volatile(addr::IE as *mut u32) & !flags.bits()); }
        // todo: also disable interrupt sources (vblank etc)
    });
}
