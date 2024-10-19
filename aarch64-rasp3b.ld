/* Declare the entrypoint */
ENTRY(_start)

SECTIONS
{
    . = 0x40000000;
    __start = .; 
    .text :
    {
        /* KEEP means no linker optimization */
        KEEP(*(.text.boot))
        *(.text._start_arguments) /* Constants (or statics in Rust speak) read by _start(). */
        *(.text._start_rust)      /* The Rust entry point */
        *(.text*)                 /* Everything else */
    } :segment_code
 
    .rodata ALIGN(16) : { 
        *(.rodata*) 
    } 
 
    .data ALIGN(16) : { 
        *(.data*) 
    } 
 
    _bss_start = .;
    .bss ALIGN(16) (NOLOAD) : { 
        *(.bss*) 
    } 
    _bss_end = .;
	. = ALIGN(16);
    . = . + 0x8000;
	_stack_top = .;
}
