OUTPUT_FORMAT("binary")
SECTIONS {
    . = 0x7C00;
    .stage0 : {
        *(.stage0 .stage0.*)
    }
    . = 0x7C00 + 510;
    .boot_magic : {
        SHORT(0xaa55)
    }
    .text : {
        *(.text .text.*)
    }
    .data : {
        *(.data .data.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .bss : {
        *(.bss .bss.*)
    }
}

