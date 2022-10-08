//! Handles the shared memory region between the ARM9 and the ARM7.

use crate::input::Buttons;

#[link_section = ".shared"]
pub static mut SHARED_DATA: SharedData = SharedData { buttons: Buttons::empty() };

pub struct SharedData {
    pub buttons: Buttons
}
