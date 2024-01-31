use core::{
    hash::{Hash, Hasher},
    sync::atomic::{self, AtomicI32},
};

use ahash::AHasher;
use alloc::vec::Vec;

use ruxtask::WaitQueueWithMetadata;

use super::BUCKET_MASK;

#[derive(Clone, Copy, Debug)]
pub(crate) struct FutexKey {
    key: usize,
    bitset: u32,
}

pub(crate) type FutexBucket = WaitQueueWithMetadata<FutexKey>;

pub(crate) struct FutexVec {
    pub(crate) buckets: Vec<FutexBucket>,
}

impl FutexKey {
    /// Create futex key from its address and a bitset.
    ///
    /// Note that `addr` or `self.key` is actually a [`PhysAddr`](memory_addr::PhysAddr) pointing to a [`i32`]
    /// but while RuxOS only supports single addr space, we'd like to treat it as a normal pointer.
    pub fn new(addr: *const i32, bitset: u32) -> Self {
        Self {
            key: addr as usize,
            bitset,
        }
    }

    /// Load the key value, atomically.
    pub fn load_val(&self) -> i32 {
        let ptr = self.key as *const AtomicI32;
        unsafe { (*ptr).load(atomic::Ordering::SeqCst) }
    }

    /// Return the address that this futex key references.
    pub fn addr(&self) -> usize {
        self.key
    }

    pub fn bitset(&self) -> u32 {
        self.bitset
    }
}

impl PartialEq for FutexKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl FutexVec {
    pub fn new(size: usize) -> Self {
        let buckets = (0..size)
            .map(|_| WaitQueueWithMetadata::new())
            .collect::<Vec<_>>();
        Self { buckets }
    }

    pub fn get_bucket(&self, key: FutexKey) -> (usize, &FutexBucket) {
        let hash = {
            // this addr should be aligned as a `*const u32`, which is this multiples of 4,
            // so ignoring the last 2 bits is fine
            let addr = key.addr() >> 2;
            let mut hasher = AHasher::default();
            addr.hash(&mut hasher);
            hasher.finish() as usize
        };
        let idx = BUCKET_MASK & hash;
        (idx, &self.buckets[idx])
    }
}


#[no_mangle]
pub unsafe extern "C" fn loggit(s: *const core::ffi::c_char) {
    log::debug!("loggit: {}", core::ffi::CStr::from_ptr(s).to_str().unwrap());
}

