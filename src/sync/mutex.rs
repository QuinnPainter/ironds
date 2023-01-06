use core::arch::asm;

pub struct RawMutex {
    locked: bool
}

impl RawMutex {
    #[must_use]
    pub const fn new() -> Self {
        RawMutex { locked: false }
    }

    fn unlock() {}

    pub fn lock() {}
}
