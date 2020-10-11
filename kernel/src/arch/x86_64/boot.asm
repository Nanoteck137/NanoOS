section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length

    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:

section .text

bits 32

global boot_entry
extern boot_entry64

boot_entry:
    mov esp, stack_top

    ; Enable pageing
    ; Setup and load a 64 bit GDT table

    call setup_page_tables
    call enable_paging
    call load_gdt

    mov word [0xb8000], 0x0f41
    jmp 0x0008:boot_entry64
    hlt

setup_page_tables:
    mov eax, p3_table
    or eax, 0b11 
    mov [p4_table], eax

    mov eax, p2_table
    or eax, 0b11
    mov [p3_table], eax

.map_p2_table:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ret

enable_paging:
    mov eax, p4_table
    mov cr3, eax

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    ret

load_gdt:
    lgdt[gdt64.pointer]
    ret

section .rodata
gdt64:
    dq 0 
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) 
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 4096
stack_top:
