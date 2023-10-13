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
    . = 0x7C00 + 510;
    .boot_magic : {
        SHORT(0xaa55)
    }
}

