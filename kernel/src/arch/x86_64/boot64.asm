section .text
bits 64

global boot_entry64

boot_entry64:
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt
