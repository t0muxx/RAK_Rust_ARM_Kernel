.globl __exception_vector_start

// Wrapper calling exception handlers
.macro CALL_EXCEPTION_HANDLER handler
__vector_\handler:
    bl disable_irq
	// wfi // wait when having an interrruping
    // using `16` causes sotring pairs of registers.
	sub	sp,  sp,  #16 * 18

	stp	x0,  x1,  [sp, #16 * 0]
	stp	x2,  x3,  [sp, #16 * 1]
	stp	x4,  x5,  [sp, #16 * 2]
	stp	x6,  x7,  [sp, #16 * 3]
	stp	x8,  x9,  [sp, #16 * 4]
	stp	x10, x11, [sp, #16 * 5]
	stp	x12, x13, [sp, #16 * 6]
	stp	x14, x15, [sp, #16 * 7]
	stp	x16, x17, [sp, #16 * 8]
	stp	x18, x19, [sp, #16 * 9]
	stp	x20, x21, [sp, #16 * 10]
	stp	x22, x23, [sp, #16 * 11]
	stp	x24, x25, [sp, #16 * 12]
	stp	x26, x27, [sp, #16 * 13]
	stp	x28, x29, [sp, #16 * 14]

	mrs	x1,  ESR_EL1
	mrs	x2,  SPSR_EL1
    mrs x3,  FAR_EL1
	mrs	x4,  ELR_EL1

	stp	lr,  x1,  [sp, #16 * 15]
	stp	x2,  x3,  [sp, #16 * 16]
    str x4, [sp, #16 * 17]

    // first handler arguments
	mov	x0,  sp

	bl	\handler

	// After returning from exception handling code, replay the saved context and return via
	// `eret`.
	b	__exception_restore_context

.size	__vector_\handler, . - __vector_\handler
.type	__vector_\handler, function
.endm

.macro FIQ_SUSPEND
1:	wfe
	b	1b
.endm


.section .text

// Vector table
// Align by 2^11 bytes, as demanded by ARMv8-A. Same as ALIGN(2048) in an ld script.
.align 11

__exception_vector_start:

.org 0x000
	CALL_EXCEPTION_HANDLER el1t_synchronous
.org 0x080
	CALL_EXCEPTION_HANDLER el1t_irq
.org 0x100
	FIQ_SUSPEND
.org 0x180
	CALL_EXCEPTION_HANDLER el1t_serror

// Current exception level with SP_ELx, x > 0.
.org 0x200
	CALL_EXCEPTION_HANDLER el1h_synchronous
.org 0x280
	CALL_EXCEPTION_HANDLER el1h_irq
.org 0x300
	FIQ_SUSPEND
.org 0x380
	CALL_EXCEPTION_HANDLER el1h_serror

// Lower exception level, AArch64
.org 0x400
	CALL_EXCEPTION_HANDLER el0_aarch64_synchronous
.org 0x480
	CALL_EXCEPTION_HANDLER el0_aarch64_irq
.org 0x500
	FIQ_SUSPEND
.org 0x580
	CALL_EXCEPTION_HANDLER el0_aarch64_serror

// Lower exception level, AArch32
.org 0x600
	CALL_EXCEPTION_HANDLER el0_aarch32_synchronous
.org 0x680
	CALL_EXCEPTION_HANDLER el0_aarch32_irq
.org 0x700
	FIQ_SUSPEND
.org 0x780
	CALL_EXCEPTION_HANDLER el0_aarch32_serror
.org 0x800


__exception_restore_context:
	ldr	w19,      [sp, #16 * 17]
	ldp	lr,  x20, [sp, #16 * 15]

	msr	ESR_EL1,  x20
	msr	ELR_EL1,  x19

	ldp	x0,  x1,  [sp, #16 * 0]
	ldp	x2,  x3,  [sp, #16 * 1]
	ldp	x4,  x5,  [sp, #16 * 2]
	ldp	x6,  x7,  [sp, #16 * 3]
	ldp	x8,  x9,  [sp, #16 * 4]
	ldp	x10, x11, [sp, #16 * 5]
	ldp	x12, x13, [sp, #16 * 6]
	ldp	x14, x15, [sp, #16 * 7]
	ldp	x16, x17, [sp, #16 * 8]
	ldp	x18, x19, [sp, #16 * 9]
	ldp	x20, x21, [sp, #16 * 10]
	ldp	x22, x23, [sp, #16 * 11]
	ldp	x24, x25, [sp, #16 * 12]
	ldp	x26, x27, [sp, #16 * 13]
	ldp	x28, x29, [sp, #16 * 14]

	add	sp,  sp,  #16 * 18
    bl enable_irq
	eret

.size	__exception_restore_context, . - __exception_restore_context
.type	__exception_restore_context, function


.globl enable_irq
enable_irq:
    msr    daifclr, #2
    ret

.globl disable_irq
disable_irq:
    msr    daifset, #2
    ret
