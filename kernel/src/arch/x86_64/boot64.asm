section .text
bits 64

global boot_entry64
extern kernel_entry

boot_entry64:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax

    call kernel_entry

    hlt
