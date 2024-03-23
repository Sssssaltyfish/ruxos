#define __SYSCALL_LL_E(x) (x)
#define __SYSCALL_LL_O(x) (x)

#define CLOBBERS  "memory", "ra", "t0", "t1", "t2", "t3", "t4", "t5", "t6"
#define CLOBBER_6 "a6"
#define CLOBBER_5 CLOBBER_6, "a5"
#define CLOBBER_4 CLOBBER_5, "a4"
#define CLOBBER_3 CLOBBER_4, "a3"
#define CLOBBER_2 CLOBBER_3, "a2"
#define CLOBBER_1 CLOBBER_2, "a1"

#define __asm_syscall(EXTRA_CLOB, ...)                                                  \
    __asm__ __volatile__("call riscv_syscall_asm\n\t" : __VA_ARGS__ : : CLOBBERS, EXTRA_CLOB); \
    return a0;

static inline long __syscall0(long n)
{
    register long a7 __asm__("a7") = n;
    register long a0 __asm__("a0");
    __asm_syscall(CLOBBER_1, "+r"(a7), "=r"(a0))
}

static inline long __syscall1(long n, long a)
{
    register long a7 __asm__("a7") = n;
    register long a0 __asm__("a0") = a;
    __asm_syscall(CLOBBER_1, "+r"(a7), "+r"(a0))
}

static inline long __syscall2(long n, long a, long b)
{
    register long a7 __asm__("a7") = n;
    register long a0 __asm__("a0") = a;
    register long a1 __asm__("a1") = b;
    __asm_syscall(CLOBBER_2, "+r"(a7), "+r"(a0), "+r"(a1))
}

static inline long __syscall3(long n, long a, long b, long c)
{
    register long a7 __asm__("a7") = n;
    register long a0 __asm__("a0") = a;
    register long a1 __asm__("a1") = b;
    register long a2 __asm__("a2") = c;
    __asm_syscall(CLOBBER_3, "+r"(a7), "+r"(a0), "+r"(a1), "+r"(a2));
}

static inline long __syscall4(long n, long a, long b, long c, long d)
{
    register long a7 __asm__("a7") = n;
    register long a0 __asm__("a0") = a;
    register long a1 __asm__("a1") = b;
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    __asm_syscall(CLOBBER_4, "+r"(a7), "+r"(a0), "+r"(a1), "+r"(a2), "+r"(a3));
}

static inline long __syscall5(long n, long a, long b, long c, long d, long e)
{
    register long a7 __asm__("a7") = n;
    register long a0 __asm__("a0") = a;
    register long a1 __asm__("a1") = b;
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    register long a4 __asm__("a4") = e;
    __asm_syscall(CLOBBER_5, "+r"(a7), "+r"(a0), "+r"(a1), "+r"(a2), "+r"(a3), "+r"(a4));
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
    register long a7 __asm__("a7") = n;
    register long a0 __asm__("a0") = a;
    register long a1 __asm__("a1") = b;
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    register long a4 __asm__("a4") = e;
    register long a5 __asm__("a5") = f;
    __asm_syscall(CLOBBER_6, "+r"(a7), "+r"(a0), "+r"(a1), "+r"(a2), "+r"(a3), "+r"(a4), "+r"(a5));
}

#define VDSO_USEFUL
/* We don't have a clock_gettime function.
#define VDSO_CGT_SYM "__vdso_clock_gettime"
#define VDSO_CGT_VER "LINUX_2.6" */

#define IPC_64 0
