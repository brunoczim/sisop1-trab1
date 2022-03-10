use std::sync::atomic::{Ordering::*, AtomicUsize};
use std::{hint, cell::UnsafeCell, marker::PhantomData};
use std::ops::{Deref, DerefMut};

const LOCKED_WRITE: usize = 0;
const UNLOCKED: usize = 1;

pub struct RwLock<T> {
    count: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> RwLock<T> {
    pub fn new(initial_value: T) -> RwLock<T> {
        RwLock {
            count: AtomicUsize::new(UNLOCKED),
            data: UnsafeCell::new(initial_value),
        }
    }
    
    pub fn try_lock_read<'lock>(&'lock self) -> Option<ReadGuard<'lock, T>> {
        let mut current = self.count.load(Relaxed);
        loop {
            if current == LOCKED_WRITE {
                return None;
            } 
            let next = current + 1;
            match self.count.compare_exchange(current, next, Acquire, Relaxed) {
                Ok(_) => return Some(ReadGuard {
                    lock: self,
                    _unsend_marker: PhantomData,
                }),
                Err(new_current) => current = new_current,
            }
        }
    }
    
    pub fn lock_read<'lock>(&'lock self) -> ReadGuard<'lock, T> {
        loop {
            match self.try_lock_read() {
                Some(guard) => return guard,
                None => hint::spin_loop(),
            }
        }
    }
    
    pub fn try_lock_write<'lock>(&'lock self) -> Option<WriteGuard<'lock, T>> {
        match self.count.compare_exchange(UNLOCKED, LOCKED_WRITE, Acquire, Relaxed) {
            Ok(_) => Some(WriteGuard {
                lock: self,
                _unsend_marker: PhantomData,
            }),
            Err(_) => None,
        }
    }
    
    pub fn lock_write<'lock>(&'lock self) -> WriteGuard<'lock, T> {
        loop {
            match self.try_lock_write() {
                Some(guard) => return guard,
                None => hint::spin_loop(),
            }
        }
    }
}

pub struct ReadGuard<'lock, T> {
    lock: &'lock RwLock<T>,
    _unsend_marker: PhantomData<*const T>
}

impl<'lock, T> Deref for ReadGuard<'lock, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'lock, T> Drop for ReadGuard<'lock, T> {
    fn drop(&mut self) {
        self.lock.count.fetch_sub(1, Release);
    }
}

pub struct WriteGuard<'lock, T> {
    lock: &'lock RwLock<T>,
    _unsend_marker: PhantomData<*mut T>
}

impl<'lock, T> Deref for WriteGuard<'lock, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'lock, T> DerefMut for WriteGuard<'lock, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'lock, T> Drop for WriteGuard<'lock, T> {
    fn drop(&mut self) {
        self.lock.count.store(UNLOCKED, Release);
    }
}
