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
    ; Setup a stack
    mov esp, stack_top

    ; Save the pointer to the multiboot strucuture
    ; This is passed on to the kernel later when we enter the kernel
    mov edi, ebx

    ; Setup a identity map of physical memory
    call setup_page_tables
    ; Enable paging
    call enable_paging
    ; Load the 64 bit GDT
    call load_gdt

    ; Long jump to boot_entry64 and from that point we can execute 
    ; 64 bit instructions
    jmp 0x0008:boot_entry64
    hlt

setup_page_tables:
    ; Set the first entry inside the p4_table to the p3_table
    mov eax, p3_table
    ; Set the present and writable bits
    or eax, 0b11 
    ; Add the entry to the first slot in the p4_table
    mov [p4_table], eax

    ; We need to the the 511th entry inside the p4 table to itself
    ; so we can later recursively map table entries later in the kernel
    mov eax, p4_table
    or eax, 0b11 
    mov [p4_table + 511 * 8], eax

    ; We do the same for the p3 table
    mov eax, p2_table
    or eax, 0b11
    mov [p3_table], eax

; Inside the p2 table we need to map all 512 entries to a physical address
.map_p2_table:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ret

; Function to enable paging
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

; Function to load the gdt
load_gdt:
    lgdt[gdt64.pointer]
    ret

section .rodata
; The GDT we load for 64 bit
gdt64:
    dq 0 
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) 
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

section .bss
; The paging tables we need for 64 bit transition
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 4096 * 4
stack_top:
