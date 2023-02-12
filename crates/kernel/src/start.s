.globl _start

.section ".text.boot"

_start:
    // https://developer.arm.com/documentation/ddi0500/e/system-control/aarch64-register-descriptions/multiprocessor-affinity-register
    mrs x0, mpidr_el1
    // checks 7th first bytes. if == 0 -> core 1.
    // This is done because by default code is execude
    // on all 4 cores.
    and x0, x0, 0xFF  
    cbz x0, master
    // if not core 0 -> hang()
    b proc_hang

proc_hang: 
    b proc_hang

.extern LD_STACK_PTR
master:
    adr    x0, master_switch_el1
    bl switch_el2

master_switch_el1:
    adr x0, master_cont
    bl switch_el1

master_cont:
    ldr    x30, =LD_STACK_PTR
    mov sp, x30
    bl    entry

.equ PSCI_SYSTEM_OFF, 0x84000008

.globl system_off
system_off:
    ldr x0, =PSCI_SYSTEM_OFF
    hvc #0
