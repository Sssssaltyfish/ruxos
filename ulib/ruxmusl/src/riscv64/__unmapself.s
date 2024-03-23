.global __unmapself
.type __unmapself, %function
__unmapself:
	li a7, 215 # SYS_munmap
	call riscv_syscall_asm
	li a7, 93  # SYS_exit
	call riscv_syscall_asm
