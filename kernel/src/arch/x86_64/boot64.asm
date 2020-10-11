section .text
bits 64

global boot_entry64
extern kernel_entry

boot_entry64:
    ; Set the segments to the null entry inside the GDT
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Call the kernel entry
    call kernel_entry

    hlt
