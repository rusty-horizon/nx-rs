// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// *Implementation adapted from `/sys/redox/condvar.rs`

use cell::UnsafeCell;
use intrinsics::atomic_cxchg;
use ptr;
use time::Duration;

use sys::mutex::{self, Mutex};
use mem;

#[cfg(target_arch = "aarch64")]
pub struct Condvar {
    lock: UnsafeCell<libnx::CondVar>,
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

#[cfg(target_arch = "aarch64")]
impl Condvar {

    pub const fn new() -> Condvar {
        Condvar {
            lock: UnsafeCell::new(0),
        }
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        self.lock = UnsafeCell::new(mem::zeroed());
    }

    #[inline]
    pub fn notify_one(&self) {
        unsafe {
            libnx::svcSignalProcessWideKey(self.lock.get(), 1);
        }
    }

    #[inline]
    pub fn notify_all(&self) {
        unsafe {
            libnx::svcSignalProcessWideKey(self.lock.get(), -1);
        }
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        self.wait_timeout(mutex, Duration::from_millis(u64::max_value()));
    }

    #[inline]
    pub fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        let dur_millis = (dur.as_secs() * 1000) + (dur.subsec_millis() as u64);
        unsafe {
            libnx::condvarWaitTimeout(self.lock.get(), mutex::raw(&mutex), dur_millis);
        }
        true
    }

    #[inline]
    pub unsafe fn destroy(&self) {
    }
}