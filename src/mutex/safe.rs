use std::sync::atomic::{Ordering::*, AtomicBool};
use std::{hint, cell::UnsafeCell, marker::PhantomData};
use std::ops::{Deref, DerefMut};

pub struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(initial_data: T) -> Mutex<T> {
        Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(initial_data)
        }
    }
    pub fn try_lock<'mtx>(&'mtx self) -> Option<MutexGuard<'mtx, T>> {
        match self.locked.compare_exchange(false, true, Acquire, Relaxed) {
           Ok(_) => Some(MutexGuard { mutex: self, _mark_unsend: PhantomData }),
           Err(_) => None,
        }
    }
    pub fn lock<'mtx>(&'mtx self) -> MutexGuard<'mtx, T> {
        loop {
            match self.try_lock() {
                Some(guard) => return guard,
                None => hint::spin_loop(),
            }
        }
    }
}

unsafe impl<T> Send for Mutex<T> where T: Send {}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

pub struct MutexGuard<'mtx, T> {
    mutex: &'mtx Mutex<T>,
    _mark_unsend: PhantomData<*mut T>,
}

impl<'mtx, T> Deref for MutexGuard<'mtx, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}
impl<'mtx, T> DerefMut for MutexGuard<'mtx, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}
impl<'mtx, T> Drop for MutexGuard<'mtx, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Release);
    }
}

unsafe impl<'mtx, T> Sync for MutexGuard<'mtx, T> where T: Sync {}
