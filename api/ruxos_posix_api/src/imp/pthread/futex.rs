/* Copyright (c) [2023] [Syswonder Community]
 *   [Ruxos] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use core::{
    ffi::{c_int, c_uint},
    time::Duration,
};

use axerrno::{ax_err, LinuxError};
use axsync::futex::syscall::{
    futex_op_and_flags_from_u32, futex_wait, futex_wait_bitset, futex_wake, futex_wake_bitset,
    FutexOp,
};

use crate::ctypes;

/// `Futex` implementation inspired by occlum
pub fn sys_futex(
    uaddr: usize,
    op: c_uint,
    val: c_int,
    // timeout value, should be struct timespec pointer
    to: usize,
    // used by Requeue
    uaddr2: c_int,
    // bitset
    val3: c_int,
) -> c_int {
    let futex_addr = uaddr as *const i32;
    let bitset = val3 as _;
    let max_count = val as _;
    let futex_val = val as _;

    syscall_body!(sys_futex, {
        let (op, _flag) = futex_op_and_flags_from_u32(op).map_err(LinuxError::from)?;
        let timeout = to as *const ctypes::timespec;
        let timeout = if !timeout.is_null()
            && matches!(op, FutexOp::FUTEX_WAIT | FutexOp::FUTEX_WAIT_BITSET)
        {
            let dur = unsafe { Duration::from(*timeout) };
            Some(dur)
        } else {
            None
        };
        debug!(
            "sys_futex <= addr: {:#x}, op: {:?}, val: {}, to: {:?}, task: {:?}, all_task: {:?}",
            uaddr, op, val, timeout, ruxtask::current().id(), ruxtask::all_task()
        );

        let ret = match op {
            FutexOp::FUTEX_WAIT => futex_wait(futex_addr, futex_val, timeout).map(|_| 0),
            FutexOp::FUTEX_WAIT_BITSET => {
                futex_wait_bitset(futex_addr, futex_val, timeout, bitset).map(|_| 0)
            }
            FutexOp::FUTEX_WAKE => futex_wake(futex_addr, max_count),
            FutexOp::FUTEX_WAKE_BITSET => futex_wake_bitset(futex_addr, max_count, bitset),
            _ => ax_err!(Unsupported, "unsupported futex option: {:?}", op),
        };
        ret.map_err(LinuxError::from)
    })
}
