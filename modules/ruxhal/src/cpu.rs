/* Copyright (c) [2023] [Syswonder Community]
 *   [Ruxos] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

//! CPU-related operations.

use percpu::PerCpu;

#[percpu::def_percpu]
static CPU_ID: usize = 0;

#[percpu::def_percpu]
static IS_BSP: bool = false;

#[percpu::def_percpu]
static CURRENT_TASK_PTR: usize = 0;

/// It is read by multiple CPUs, but only write once in `init`(`_secondary`),
/// and fences are employed to prevent race condition. It just stores `percpu_area_base`,
/// as a performance improvement to ease the need of calculating `percpu_area_base`
/// every time.
static mut PERCPU_BASES: [usize; ruxconfig::SMP] = [0; ruxconfig::SMP];

/// Returns the ID of the current CPU.
#[inline]
pub fn this_cpu_id() -> usize {
    CPU_ID.read_current()
}

/// Returns whether the current CPU is the primary CPU (aka the bootstrap
/// processor or BSP)
#[inline]
pub fn this_cpu_is_bsp() -> bool {
    IS_BSP.read_current()
}

/// Gets the pointer to the current task with preemption-safety.
///
/// Preemption may be enabled when calling this function. This function will
/// guarantee the correctness even the current task is preempted.
#[inline]
pub fn current_task_ptr<T>() -> *const T {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // on x86, only one instruction is needed to read the per-CPU task pointer from `gs:[off]`.
        CURRENT_TASK_PTR.read_current_raw() as _
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        // on RISC-V, reading `CURRENT_TASK_PTR` requires multiple instruction, so we disable local IRQs.
        let _guard = kernel_guard::IrqSave::new();
        CURRENT_TASK_PTR.read_current_raw() as _
    }
    #[cfg(target_arch = "aarch64")]
    {
        // on ARM64, we use `SP_EL0` to store the task pointer.
        use tock_registers::interfaces::Readable;
        aarch64_cpu::registers::SP_EL0.get() as _
    }
}

/// Sets the pointer to the current task with preemption-safety.
///
/// Preemption may be enabled when calling this function. This function will
/// guarantee the correctness even the current task is preempted.
///
/// # Safety
///
/// The given `ptr` must be pointed to a valid task structure.
#[inline]
pub unsafe fn set_current_task_ptr<T>(ptr: *const T) {
    #[cfg(target_arch = "x86_64")]
    {
        CURRENT_TASK_PTR.write_current_raw(ptr as usize)
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    {
        let _guard = kernel_guard::IrqSave::new();
        CURRENT_TASK_PTR.write_current_raw(ptr as usize)
    }
    #[cfg(target_arch = "aarch64")]
    {
        use tock_registers::interfaces::Writeable;
        aarch64_cpu::registers::SP_EL0.set(ptr as u64)
    }
}

#[inline]
pub fn get_percpu_base(cpu_id: usize) -> usize {
    unsafe { PERCPU_BASES[cpu_id] }
}

#[inline]
pub fn get_current_percpu_base() -> usize {
    unsafe { get_percpu_base(CPU_ID.read_current_raw()) }
}

#[inline]
pub fn get_percpu_ptr_on<T: PerCpu>(t: &T, cpu_id: usize) -> *mut T::Type {
    unsafe { (t.offset() + PERCPU_BASES[cpu_id]) as _ }
}

#[allow(dead_code)]
pub(crate) fn init_primary(cpu_id: usize) {
    percpu::init(ruxconfig::SMP);
    percpu::set_local_thread_pointer(cpu_id);
    unsafe {
        CPU_ID.write_current_raw(cpu_id);
        IS_BSP.write_current_raw(true);
        PERCPU_BASES[cpu_id] = percpu::percpu_area_base(cpu_id);
    }
    core::sync::atomic::fence(core::sync::atomic::Ordering::AcqRel);
}

#[allow(dead_code)]
pub(crate) fn init_secondary(cpu_id: usize) {
    percpu::set_local_thread_pointer(cpu_id);
    unsafe {
        CPU_ID.write_current_raw(cpu_id);
        IS_BSP.write_current_raw(false);
        PERCPU_BASES[cpu_id] = percpu::percpu_area_base(cpu_id);
    }
    core::sync::atomic::fence(core::sync::atomic::Ordering::AcqRel);
}
