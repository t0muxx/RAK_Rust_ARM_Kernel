ENTRY(_start)
SECTIONS
{
    . = 0x80000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : { *(.bss) }

    . = ALIGN(32);
    . = . + 0x4000;
    LD_STACK_PTR = .;
}
