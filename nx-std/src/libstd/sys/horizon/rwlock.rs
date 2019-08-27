// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#[cfg(target_arch = "aarch64")]
pub use self::hos::*;
#[cfg(target_arch = "aarch64")]
mod hos {
    use mem;
    use cell::UnsafeCell;

    // TODO: properly implement this
    
    pub struct RWLock {
        // inner : UnsafeCell<sys::RwLock>
    }
    
    unsafe impl Send for RWLock {}
    unsafe impl Sync for RWLock {}

    impl RWLock {
        pub const fn new() -> RWLock {
            RWLock {
            }
        }
        
        #[inline]
        pub unsafe fn read(&self) {
            // sys::rwlockReadLock(self.inner.get());
        }
        
        #[inline]
        pub unsafe fn write(&self) {
            // sys::rwlockWriteLock(self.inner.get());
        }
        
        #[inline]
        pub unsafe fn read_unlock(&self) {
            // sys::rwlockReadUnlock(self.inner.get());
        }

        #[inline]
        pub unsafe fn write_unlock(&self) {
            // sys::rwlockWriteUnlock(self.inner.get());
        }

        #[inline]
        pub unsafe fn try_read(&self) -> bool {
            false
            /*
            let raw_ptr = &mut *self.inner.get();
            if !sys::rmutexTryLock(&mut raw_ptr.r as *mut sys::RMutex) {
                return false;
            }

            raw_ptr.b += 1;
            if raw_ptr.b == 0 {
                sys::rmutexLock(&mut raw_ptr.g as *mut sys::RMutex);
            }
            sys::rmutexUnlock(&mut raw_ptr.r as *mut sys::RMutex);
            true
            */
        }
        
        #[inline]
        pub unsafe fn try_write(&self) -> bool {
            false
            /*
            let raw_ptr = &mut *self.inner.get();
            sys::rmutexTryLock(&mut raw_ptr.g as *mut sys::RMutex)
            */
        }

        #[inline]
        pub unsafe fn destroy(&self) {
        }

    } 
}
