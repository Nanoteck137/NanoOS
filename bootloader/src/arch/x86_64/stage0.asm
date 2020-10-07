[org  0x7c00]
[bits 16]

entry:
    cli
    cld

    mov [boot_disk], dl

    ; Load some space
    mov ah, 0x02
    mov bx, bootloader_entry
    mov al, 1 ; More?
    mov dl, [boot_disk]
    mov ch, 0
    mov dh, 0x00
    mov cl, 2 ; Read from sector 2, one sector after the boot sector

    int 0x13
    ; Check for errors

	in    al, 0x92
	or    al, 2
	out 0x92, al

    xor ax, ax
    mov ds, ax

    lgdt [ds:pm_gdt]

	mov eax, cr0
	or  eax, (1 << 0)
	mov cr0, eax

    jmp 0x0008:pm_entry

[bits 32]

pm_entry:
    mov ax, 0x10
    mov es, ax
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov esp, 0x7c00

    call bootloader_entry

    jmp $

align 8
pm_gdt_base:
	dq 0x0000000000000000
	dq 0x00CF9A000000FFFF
	dq 0x00CF92000000FFFF

pm_gdt:
	dw (pm_gdt - pm_gdt_base) - 1
	dd pm_gdt_base

boot_disk:
    db 0

times 510 - ($ - $$) db 0
dw 0xaa55

bootloader_entry:
incbin "build/test.bin"
