use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use crate::sync::NdsCell;

pub struct NdsMutex<T> {
    locked: NdsCell<bool>,
    data: UnsafeCell<T>
}
unsafe impl<T> Send for NdsMutex<T> {}
unsafe impl<T> Sync for NdsMutex<T> {}

impl<T> NdsMutex<T> {
    #[inline]
    #[must_use]
    pub const fn new(in_data: T) -> Self {
        Self {
            locked: NdsCell::new(false),
            data: UnsafeCell::new(in_data)
        }
    }

    #[inline]
    fn unlock(&self) {
        if !self.locked.swap(false) {
            panic!("Tried to unlock a mutex that wasn't locked");
        }
    }

    #[inline]
    #[must_use]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        if self.locked.swap(true) {
            panic!("Tried to lock a mutex that was already locked");
        }
        MutexGuard(self)
    }

    #[inline]
    #[must_use]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.locked.swap(true) {
            None
        } else {
            Some(MutexGuard(self))
        }
    }
}

#[must_use]
pub struct MutexGuard<'a, T>(&'a NdsMutex<T>);
impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.0.unlock();
    }
}
impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.data.get() }
    }
}
impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.data.get() }
    }
}
