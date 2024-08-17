; x86 bios loads us in at 0x7C00, here we tell nasm to use that
; for addressing
;ORG 0x7C00

extern entry
global invoke_realmode

section .stage0

%define A20_PORT 0x92

; assemble copatable with x86 real mode
[bits 16]
boot_entry: 
    ; Disable hardware interupts
    cli

    ; 16 bit clear screen interupt
    mov ax, 0x03
    int 0x10

    ;call enable_draw_mode

    ; Set A20
    in al, A20_PORT
    or al, 2
    out A20_PORT, al

    ; Get the memory map from bios for our allocator later
    call get_memory_map

    ; Read next stage bootloader from disk
    call read_disk

    ; Load our Global Descriptor Table to disable segmentation memory 
    ; management
    ; https://c9x.me/x86/html/file_module_x86_id_156.html
    lgdt [gdt_desc]

    ; Enable protected mode
    ; https://wiki.osdev.org/CPU_Registers_x86#CR0
    mov eax, cr0
    or  eax, (1 << 0)
    mov cr0, eax

    ; Jump to 32 bit mode with the code descriptor as cs cant be modified with mov
    ; https://stackoverflow.com/questions/23978486/far-jump-in-gdt-in-bootloader
    jmp (gdt_protected_code - gdt_base):protected_entry

; Enable 256 Colour draw mode
enable_draw_mode:
    mov ax, 0x13
    int 0x10
    ret

get_memory_map:

.first_run:
    mov di, memory_map; Give me memory_map entries to this address

    mov edx, "PAMS" ; Magic
    mov eax, 0xE820 ; Function name
    mov [es:di + 20], dword 1
    mov ecx, 24 ; Ask for 24 bytes
    int 0x15

    ; Error
    jc short .end_get_memory_map

    ; Check for end
    test ebx, ebx
    je short .end_get_memory_map

    ; Do while cl != 0 (CL is the returned bytes)
    jcxz .end_get_memory_map

.loop:
    add di, 24

    mov edx, "PAMS" ; Magic
    mov eax, 0xE820 ; Function name
    mov [es:di + 20], dword 1
    mov ecx, 24 ; Ask for 24 bytes
    int 0x15

    ; Check for end
    test ebx, ebx
    jne short .loop

.end_get_memory_map:
    ret

read_disk:
     ; Load the additional bootloader code from disk
    mov ah, 0x42
    mov si, disk_access_packet
    mov dl, byte 0x80
    int 0x13
    ret

; Struct we pass to int 0x13 to read from disk
disk_access_packet:
    size db 16
    reserved db 0
    ; 128 is the max allowed
    max_sectors dw 128 
    ; We write the whole disk to the entrypoint over the top of what we have
    load_address dd 0x7C00 
    start_sector dq 0

; Run a real mode function in here
realmode:
    mov eax, cr0
    and eax, ~1
    mov cr0, eax

    xor ax, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Need to swtich CS to the base addres (0) with farjump
    jmp 0:.enter_func

.enter_func:
    mov eax, [esp + 8]
    mov bx, [eax + Registers.bx]
    mov cx, [eax + Registers.cx]
    mov dx, [eax + Registers.dx]
    mov ax, [eax + Registers.ax]
    int 0x10

;	jmp $

.exit_func:
    mov eax, cr0
    or eax, 1
    mov cr0, eax

	mov ax, (gdt_protected_data - gdt_base)
	mov es, ax
	mov ds, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

;    pushfd
;    push dword (gdt_protected_code - gdt_base)
;    push dword realmode_exit
;    iretd
    jmp (gdt_protected_code - gdt_base):realmode_exit 

; Lets us Read/Write/Execute all memory! God mode!
; https://wiki.osdev.org/GDT_Tutorial
; https://wiki.osdev.org/GDT
gdt_base:
; Section0 - Null Descriptor, required for sanity 
gdt_protected_null:
    dq 0x0000000000000000
gdt_real_code:
    ; Limit
    dw 0xFFFF
    ; Base
    dw 0x0000
    ; BaseHigher
    db 0x00
    ; AccessByte
    db 0b10011010
    ; Flags/Limit
    db 0x00
    ; Base
    db 0x00
gdt_real_data:   
    ; Limit
    dw 0xFFFF
    ; BaseLower
    dw 0x0000
    ; BaseHigher
    db 0x00
    ; AccessByte
    db 0b10010010
    ; Flags/Limit
    db 0x00
    ; Base
    db 0x00
gdt_protected_code:
    ; Limit
    dw 0xFFFF
    ; Base
    dw 0x0000
    ; BaseHigher
    db 0x00
    ; AccessByte
    db 0b10011010
    ; Flags/Limit
    db 0xCF
    ; Base
    db 0x00
gdt_protected_data:   
    ; Limit
    dw 0xFFFF
    ; BaseLower
    dw 0x0000
    ; BaseHigher
    db 0x00
    ; AccessByte
    db 0b10010010
    ; Flags/Limit
    db 0xCF
    ; Base
    db 0x00

gdt_desc:
    ; GDT.Limit
    dw gdt_desc - gdt_base - 1
    ; GDT.Base
    dd gdt_base

[bits 32]
protected_entry:
    ; Reload the data segment registers with the data descriptor
    ; First we calculate the offset of the data segment in GDT 
    mov   ax, (gdt_protected_data - gdt_base)
    mov   ds, ax
    mov   es, ax
    mov   fs, ax
    mov   gs, ax
    mov   ss, ax

    ; Set up a stack
    mov esp, 0x7C00

    ; Pass GDT code selector we want to use
    push (gdt_protected_code - gdt_base)
    ; Pass memory mem_map address to rust
    push memory_map
    ; Pass the entry address of rust
    push entry
    ; fn entry(memory_map: u32, entry_addr: u32, gdt_cs_offset: u16)
    call entry

; Args
; Registers: usize - [ESP+8] - Struct of register values to be used in the call
; Int: u16 - [ESP+4]- The interrupt to be invoked
invoke_realmode:
    cli
	pushad
	add esp, 8 * 4

    mov ax, gdt_real_data - gdt_base
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    jmp (gdt_real_code - gdt_base):realmode

realmode_exit:
	sub esp, 8 * 4
	popad
    sti
    ret

; Rust struct
struc Registers
    .ax: resw 1 
    .bx: resw 1
    .cx: resw 1
    .dx: resw 1
endstruc
; Reserve space for entries in the memory map
memory_map: equ 0x400
