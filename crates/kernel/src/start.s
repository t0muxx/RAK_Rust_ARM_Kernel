.globl _start

.section ".text.boot"

_start:
    // https://developer.arm.com/documentation/ddi0500/e/system-control/aarch64-register-descriptions/multiprocessor-affinity-register
    mrs x0, mpidr_el1
    // checks 7th first bytes. if == 0 -> core 1.
    // This is done because by default code is execude
    // on all 4 cores.
    and x0, x0, 0xFF  
    cbz x0, zero_bss
    // if not core 0 -> hang()
    b proc_hang

proc_hang: 
    b proc_hang

zero_bss:
    /* zero bss */ 
    adr x4, _bss_start
    adr x9, _bss_end
    mov x5, 0
    mov x6, 0
    mov x7, 0
    mov x8, 0
    b       cmp_bss
loop_bss:
    str x5, [x4]
    add x4, x4, 4
cmp_bss:
    cmp x4, x9
    blo loop_bss

    b master

master:
    /* set exception vector and switch exception level */
    adr x0, __exception_vector_start
    msr vbar_el1, x0
	/* The following two instruction are not needed as starting
		qemu with `-kernel` loading start our code in EL1 :
    adr    x0, master_switch_el1 
	bl switch_el2 
	*/

	/* The following two instruction are not needed as starting
		qemu with `-kernel` loading start our code in EL1 :
master_switch_el1:
     adr x0, master_cont
     bl switch_el1 -> not need as qemu virt starts in el1
	*/

master_cont:
    ldr x30, =_stack_top
	mov sp, x30
    bl    entry
