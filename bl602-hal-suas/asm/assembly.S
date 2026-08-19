/*
Inspired from https://github.com/sipeed/bl602-hal/blob/main/trap.S
*/

#define STORE    sw
#define LOAD     lw
#define LOG_REGBYTES 2
#define REGBYTES (1 << LOG_REGBYTES)

/*
    Enable Floating-Point Unit
    See https://people.eecs.berkeley.edu/~krste/papers/riscv-privileged-v1.9.1.pdf ; pp. 24-25
*/

.text
.global _enable_fpu

_enable_fpu:
    li t0, 0x00006000 // Load value to enable FPU into t0
    csrs mstatus, t0  // Store t0 into mstatus to enable FPU
    csrr t1, mstatus  // Read mstatus into t1
    and t1, t1, t0    // Perform logical AND to extract the current status of the FPU
    beqz t1, 1f       // Jump if t1 is zero, meaning the FPU is still disabled
    fssr zero         // Clear floating point status register
1:
    ret               // Return

/*
    Trap entry point (_start_trap)

    Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
    restores caller saved registers and then returns.
*/
.section .trap, "ax"
.global _start_trap_hal
.option norelax
.align 6

_start_trap_hal:
    // Store registers
    addi sp, sp, -32*REGBYTES

    STORE ra, 0*REGBYTES(sp)
    STORE t0, 1*REGBYTES(sp)
    STORE t1, 2*REGBYTES(sp)
    STORE t2, 3*REGBYTES(sp)
    STORE t3, 4*REGBYTES(sp)
    STORE t4, 5*REGBYTES(sp)
    STORE t5, 6*REGBYTES(sp)
    STORE t6, 7*REGBYTES(sp)
    STORE a0, 8*REGBYTES(sp)
    STORE a1, 9*REGBYTES(sp)
    STORE a2, 10*REGBYTES(sp)
    STORE a3, 11*REGBYTES(sp)
    STORE a4, 12*REGBYTES(sp)
    STORE a5, 13*REGBYTES(sp)
    STORE a6, 14*REGBYTES(sp)
    STORE a7, 15*REGBYTES(sp)
    STORE s0, 16*REGBYTES(sp)
    STORE s1, 17*REGBYTES(sp)
    STORE s2, 18*REGBYTES(sp)
    STORE s3, 19*REGBYTES(sp)
    STORE s4, 20*REGBYTES(sp)
    STORE s5, 21*REGBYTES(sp)
    STORE s6, 22*REGBYTES(sp)
    STORE s7, 23*REGBYTES(sp)
    STORE s8, 24*REGBYTES(sp)
    STORE s9, 25*REGBYTES(sp)
    STORE s10, 26*REGBYTES(sp)
    STORE s11, 27*REGBYTES(sp)
    STORE gp, 28*REGBYTES(sp)
    STORE tp, 29*REGBYTES(sp)

    // Store stack pointer
    addi s0, sp, 32*REGBYTES
    STORE s0, 30*REGBYTES(sp)

    // Store FPU registers
    addi sp, sp, -(34 * REGBYTES) // Make room for the additional FPU registers.

    fsw   f0,   1*REGBYTES(sp)
    fsw   f1,   2*REGBYTES(sp)
    fsw   f2,   3*REGBYTES(sp)
    fsw   f3,   4*REGBYTES(sp)
    fsw   f4,   5*REGBYTES(sp)
    fsw   f5,   6*REGBYTES(sp)
    fsw   f6,   7*REGBYTES(sp)
    fsw   f7,   8*REGBYTES(sp)
    fsw   f8,   9*REGBYTES(sp)
    fsw   f9,  10*REGBYTES(sp)
    fsw   f10, 11*REGBYTES(sp)
    fsw   f11, 12*REGBYTES(sp)
    fsw   f12, 13*REGBYTES(sp)
    fsw   f13, 14*REGBYTES(sp)
    fsw   f14, 15*REGBYTES(sp)
    fsw   f15, 16*REGBYTES(sp)
    fsw   f16, 17*REGBYTES(sp)
    fsw   f17, 18*REGBYTES(sp)
    fsw   f18, 19*REGBYTES(sp)
    fsw   f19, 20*REGBYTES(sp)
    fsw   f20, 21*REGBYTES(sp)
    fsw   f21, 22*REGBYTES(sp)
    fsw   f22, 23*REGBYTES(sp)
    fsw   f23, 24*REGBYTES(sp)
    fsw   f24, 25*REGBYTES(sp)
    fsw   f25, 26*REGBYTES(sp)
    fsw   f26, 27*REGBYTES(sp)
    fsw   f27, 28*REGBYTES(sp)
    fsw   f28, 29*REGBYTES(sp)
    fsw   f29, 30*REGBYTES(sp)
    fsw   f30, 31*REGBYTES(sp)
    fsw   f31, 32*REGBYTES(sp)

    // Store FPU status
    frcsr t0  
    sw    t0,  33*REGBYTES(sp)

    // Call interrupt service routine
    add a0, sp, zero
    jal ra, _start_trap_rust_hal

    // Restore FPU registers
    flw   f0,   1*REGBYTES(sp)
    flw   f1,   2*REGBYTES(sp)
    flw   f2,   3*REGBYTES(sp)
    flw   f3,   4*REGBYTES(sp)
    flw   f4,   5*REGBYTES(sp)
    flw   f5,   6*REGBYTES(sp)
    flw   f6,   7*REGBYTES(sp)
    flw   f7,   8*REGBYTES(sp)
    flw   f8,   9*REGBYTES(sp)
    flw   f9,  10*REGBYTES(sp)
    flw   f10, 11*REGBYTES(sp)
    flw   f11, 12*REGBYTES(sp)
    flw   f12, 13*REGBYTES(sp)
    flw   f13, 14*REGBYTES(sp)
    flw   f14, 15*REGBYTES(sp)
    flw   f15, 16*REGBYTES(sp)
    flw   f16, 17*REGBYTES(sp)
    flw   f17, 18*REGBYTES(sp)
    flw   f18, 19*REGBYTES(sp)
    flw   f19, 20*REGBYTES(sp)
    flw   f20, 21*REGBYTES(sp)
    flw   f21, 22*REGBYTES(sp)
    flw   f22, 23*REGBYTES(sp)
    flw   f23, 24*REGBYTES(sp)
    flw   f24, 25*REGBYTES(sp)
    flw   f25, 26*REGBYTES(sp)
    flw   f26, 27*REGBYTES(sp)
    flw   f27, 28*REGBYTES(sp)
    flw   f28, 29*REGBYTES(sp)
    flw   f29, 30*REGBYTES(sp)
    flw   f30, 31*REGBYTES(sp)
    flw   f31, 32*REGBYTES(sp)

    // Restore FPU status
    lw    t0,  33*REGBYTES(sp)
    fscsr t0

    addi sp, sp, (34 * REGBYTES) // Remove space added for additional FPU registers.

    // Restore registers
    LOAD ra, 0*REGBYTES(sp)
    LOAD t0, 1*REGBYTES(sp)
    LOAD t1, 2*REGBYTES(sp)
    LOAD t2, 3*REGBYTES(sp)
    LOAD t3, 4*REGBYTES(sp)
    LOAD t4, 5*REGBYTES(sp)
    LOAD t5, 6*REGBYTES(sp)
    LOAD t6, 7*REGBYTES(sp)
    LOAD a0, 8*REGBYTES(sp)
    LOAD a1, 9*REGBYTES(sp)
    LOAD a2, 10*REGBYTES(sp)
    LOAD a3, 11*REGBYTES(sp)
    LOAD a4, 12*REGBYTES(sp)
    LOAD a5, 13*REGBYTES(sp)
    LOAD a6, 14*REGBYTES(sp)
    LOAD a7, 15*REGBYTES(sp)
    LOAD s0, 16*REGBYTES(sp)
    LOAD s1, 17*REGBYTES(sp)
    LOAD s2, 18*REGBYTES(sp)
    LOAD s3, 19*REGBYTES(sp)
    LOAD s4, 20*REGBYTES(sp)
    LOAD s5, 21*REGBYTES(sp)
    LOAD s6, 22*REGBYTES(sp)
    LOAD s7, 23*REGBYTES(sp)
    LOAD s8, 24*REGBYTES(sp)
    LOAD s9, 25*REGBYTES(sp)
    LOAD s10, 26*REGBYTES(sp)
    LOAD s11, 27*REGBYTES(sp)
    LOAD gp, 28*REGBYTES(sp)
    LOAD tp, 29*REGBYTES(sp)

    // Restore SP
    LOAD sp, 30*REGBYTES(sp)

    # SP was restored from the original SP
    // Return
    mret
