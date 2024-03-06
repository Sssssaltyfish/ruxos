use core::time::Duration;

use axerrno::{ax_err, ax_err_type, AxResult};
use axlog::debug;
use bitflags::bitflags;

use super::{
    types::{FutexBucket, FutexKey},
    FUTEX_BUCKETS,
};

const FUTEX_OP_MASK: u32 = 0x0000_000F;
const FUTEX_FLAGS_MASK: u32 = u32::MAX ^ FUTEX_OP_MASK;
const FUTEX_BITSET_MATCH_ANY: u32 = u32::MAX;

#[derive(PartialEq, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FutexOp {
    FUTEX_WAIT = 0,
    FUTEX_WAKE = 1,
    FUTEX_FD = 2,
    FUTEX_REQUEUE = 3,
    FUTEX_CMP_REQUEUE = 4,
    FUTEX_WAKE_OP = 5,
    FUTEX_LOCK_PI = 6,
    FUTEX_UNLOCK_PI = 7,
    FUTEX_TRYLOCK_PI = 8,
    FUTEX_WAIT_BITSET = 9,
    FUTEX_WAKE_BITSET = 10,
}

bitflags! {
    pub struct FutexFlags : u32 {
        const FUTEX_PRIVATE         = 128;
        const FUTEX_CLOCK_REALTIME  = 256;
    }
}

impl FutexOp {
    pub fn from_u32(bits: u32) -> AxResult<FutexOp> {
        match bits {
            0 => Ok(FutexOp::FUTEX_WAIT),
            1 => Ok(FutexOp::FUTEX_WAKE),
            2 => Ok(FutexOp::FUTEX_FD),
            3 => Ok(FutexOp::FUTEX_REQUEUE),
            4 => Ok(FutexOp::FUTEX_CMP_REQUEUE),
            5 => Ok(FutexOp::FUTEX_WAKE_OP),
            6 => Ok(FutexOp::FUTEX_LOCK_PI),
            7 => Ok(FutexOp::FUTEX_UNLOCK_PI),
            8 => Ok(FutexOp::FUTEX_TRYLOCK_PI),
            9 => Ok(FutexOp::FUTEX_WAIT_BITSET),
            10 => Ok(FutexOp::FUTEX_WAKE_BITSET),
            _ => return ax_err!(InvalidInput, "unknown futex op: {}", bits),
        }
    }
}

impl FutexFlags {
    pub fn from_u32(bits: u32) -> AxResult<FutexFlags> {
        FutexFlags::from_bits(bits)
            .ok_or_else(|| ax_err_type!(InvalidInput, "unknown futex flags: {}", bits))
    }
}

pub fn futex_op_and_flags_from_u32(bits: u32) -> AxResult<(FutexOp, FutexFlags)> {
    let op = {
        let op_bits = bits & FUTEX_OP_MASK;
        FutexOp::from_u32(op_bits)?
    };
    let flags = {
        let flags_bits = bits & FUTEX_FLAGS_MASK;
        FutexFlags::from_u32(flags_bits)?
    };
    Ok((op, flags))
}

pub fn futex_wait(
    futex_addr: *const i32,
    futex_val: i32,
    timeout: Option<Duration>,
) -> AxResult<()> {
    futex_wait_bitset(futex_addr, futex_val, timeout, FUTEX_BITSET_MATCH_ANY)
}

pub fn futex_wait_bitset(
    futex_addr: *const i32,
    futex_val: i32,
    timeout: Option<Duration>,
    bitset: u32,
) -> AxResult<()> {
    debug!(
        "futex_wait_bitset addr: {:#x}, val: {}, timeout: {:?}, bitset: {:#x}",
        futex_addr as usize, futex_val, timeout, bitset
    );
    // Get and lock the futex bucket
    let futex_key = FutexKey::new(futex_addr, bitset);
    let (_, futex_bucket) = FUTEX_BUCKETS.get_bucket(futex_key);

    let condition = || {
        // Check the futex value
        let actual_val = futex_key.load_val();
        trace!("futex_wait_bitset actual_val: {}", actual_val);
        if actual_val != futex_val {
            return ax_err!(
                WouldBlock,
                "futex value does not match: expected {}, found {}",
                futex_val,
                actual_val
            );
        }

        Ok(())
    };

    // Lock the queue before checking futex value.
    match timeout {
        Some(timeout) => {
            #[cfg(feature = "irq")]
            let wait_timeout = if bitset == FUTEX_BITSET_MATCH_ANY {
                FutexBucket::wait_timeout_absolutely_meta_if
            } else {
                FutexBucket::wait_timeout_meta_if
            };
            #[cfg(not(feature = "irq"))]
            let wait_timeout = FutexBucket::wait_timeout_absolutely_meta_if;
            let _is_timeout = wait_timeout(futex_bucket, timeout, futex_key, condition)?;
            Ok(())
        }
        None => futex_bucket.wait_meta_if(futex_key, condition),
    }
}

pub fn futex_wake(futex_addr: *const i32, max_count: usize) -> AxResult<usize> {
    futex_wake_bitset(futex_addr, max_count, FUTEX_BITSET_MATCH_ANY)
}

pub fn futex_wake_bitset(futex_addr: *const i32, max_count: usize, bitset: u32) -> AxResult<usize> {
    debug!(
        "futex_wake_bitset addr: {:#x}, max_count: {}, bitset: {:#x}",
        futex_addr as usize, max_count, bitset
    );

    let futex_key = FutexKey::new(futex_addr, bitset);
    let (_, futex_bucket) = FUTEX_BUCKETS.get_bucket(futex_key);

    let mut count = 0;

    // Wake up the tasks in the bucket
    let task_count = futex_bucket.notify_task_if(false, |task, &key| {
        trace!(
            "futex wake: count: {}, key: {:?}, futex_key: {:?}, bitset: {}, is_notified: {}, task: {:?}",
            count,
            key,
            futex_key,
            bitset,
            !(count >= max_count || futex_key != key || (bitset & key.bitset()) == 0),
            task,
        );
        if !task.is_blocked() {
            return true;
        }
        if count >= max_count || futex_key != key || (bitset & key.bitset()) == 0 {
            false
        } else {
            count += 1;
            true
        }
    });
    Ok(task_count)
}
