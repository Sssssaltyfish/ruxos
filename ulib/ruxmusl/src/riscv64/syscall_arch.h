#define __SYSCALL_LL_E(x) (x)
#define __SYSCALL_LL_O(x) (x)

struct RvSyscallArgs {
    long a0, a1, a2, a3, a4, a5;
};

extern long riscv_syscall_entry(long, struct RvSyscallArgs);

static inline long __syscall0(long n)
{
    struct RvSyscallArgs args = {0};
    return riscv_syscall_entry(n, args);
}

static inline long __syscall1(long n, long a)
{
    struct RvSyscallArgs args = {0};
    args.a0 = a;
    return riscv_syscall_entry(n, args);
}

static inline long __syscall2(long n, long a, long b)
{
    struct RvSyscallArgs args = {0};
    args.a0 = a;
    args.a1 = b;
    return riscv_syscall_entry(n, args);
}

static inline long __syscall3(long n, long a, long b, long c)
{
    struct RvSyscallArgs args = {0};
    args.a0 = a;
    args.a1 = b;
    args.a2 = c;
    return riscv_syscall_entry(n, args);
}

static inline long __syscall4(long n, long a, long b, long c, long d)
{
    struct RvSyscallArgs args = {0};
    args.a0 = a;
    args.a1 = b;
    args.a2 = c;
    args.a3 = d;
    return riscv_syscall_entry(n, args);
}

static inline long __syscall5(long n, long a, long b, long c, long d, long e)
{
    struct RvSyscallArgs args = {0};
    args.a0 = a;
    args.a1 = b;
    args.a2 = c;
    args.a3 = d;
    args.a4 = e;
    return riscv_syscall_entry(n, args);
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
    struct RvSyscallArgs args = {0};
    args.a0 = a;
    args.a1 = b;
    args.a2 = c;
    args.a3 = d;
    args.a4 = e;
    args.a5 = f;
    return riscv_syscall_entry(n, args);
}

#define VDSO_USEFUL
/* We don't have a clock_gettime function.
#define VDSO_CGT_SYM "__vdso_clock_gettime"
#define VDSO_CGT_VER "LINUX_2.6" */

#define IPC_64 0
