; x86 bios loads us in at 0x7C00, here we tell nasm to use that
; for addressing
;ORG 0x7C00

section .text.asm.entry
    extern entry

; assemble copatable with x86 real mode
[bits 16]
real_entry: 
    ; Disable hardware interupts
    cli

    ; 16 bit clear screen interupt
    mov ax, 0x03
    int 0x10

    ; Set A20
    in al, 0x92
    or al, 2
    out 0x92, al
    ; Load our Global Descriptor Table to disable segmentation memory management
    ; https://c9x.me/x86/html/file_module_x86_id_156.html
    lgdt [global_desc_table_desc]

    ; Enable protected mode
    ; https://wiki.osdev.org/CPU_Registers_x86#CR0
	mov eax, cr0
	or  eax, (1 << 0)
	mov cr0, eax

    ; Jump to 32 bit mode with the code descriptor as cs cant be modified with mov
    ; https://stackoverflow.com/questions/23978486/far-jump-in-gdt-in-bootloader
    jmp (gdt_section_code - global_desc_table_base):pm_entry

; Lets us Read/Write/Execute all memory! God mode!
; https://wiki.osdev.org/GDT_Tutorial
; https://wiki.osdev.org/GDT
global_desc_table_base:
; Section0 - Null Descriptor, required for sanity 
gdt_section_null:
    dq 0x0000000000000000
gdt_section_code:
    ; Limit
    dw 0xFFFF
	; Base
    dw 0x0000
    db 0x00
    ; AccessByte
    db 0b10011010
    ; Flags/Limit
    db 0xCF
    ; Base
	db 0x00
gdt_section_data:	
    ; Limit
    dw 0xFFFF
	; Base
    dw 0x0000
    db 0x00
    ; AccessByte
    db 0b10010010
    ; Flags/Limit
    db 0xCF
    ; Base
	db 0x00

global_desc_table_desc:
    ; GDT.Limit
    dw global_desc_table_desc - global_desc_table_base - 1
    ; GDT.Base
    dd global_desc_table_base


[bits 32]
pm_entry:
    ; Reload the data segment registers with the data descriptor
    mov   ax, (gdt_section_data - global_desc_table_base) ; 0X10 IS A STAND-IN FOR YOUR DATA SEGMENT
    mov   ds, ax
    mov   es, ax
    mov   fs, ax
    mov   gs, ax
    mov   ss, ax

    ; Write to VGA buffer
    ;mov eax, 0x07690748
    ;mov [0x0B8000], eax
    mov esp, 0x7C00
    call entry

