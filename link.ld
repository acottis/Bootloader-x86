OUTPUT_FORMAT("binary")
SECTIONS {
    . = 0x7C00;
    .text : {
        *(.text.asm.entry)
        *(.text.entry)
        *(.text)
    }
    .data : {
        *(.data)
    }
    .rodata : {
        *(.rodata)
    }
    .bss : {
        *(.bss)
    }
    . = 0x7DFE;
    .text : {
        *(.boot.bytes)
    }
}

