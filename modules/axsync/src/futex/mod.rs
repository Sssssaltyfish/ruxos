use lazy_init::LazyInit;
use types::{FutexBucket, FutexKey, FutexVec};

pub mod syscall;
mod types;

// Use the same count as linux kernel to keep the same performance
const BUCKET_COUNT: usize = ((1 << 8) * (ruxconfig::SMP)).next_power_of_two();
const BUCKET_MASK: usize = BUCKET_COUNT - 1;
static FUTEX_BUCKETS: LazyInit<FutexVec> = LazyInit::new();

pub fn init_futex() {
    FUTEX_BUCKETS.init_by(FutexVec::new(BUCKET_COUNT));
}
