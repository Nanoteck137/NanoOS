[org  0x7c00]
[bits 16]

entry:
    cli
    cld

    mov [boot_disk], dl

    ; Load some space
    mov ah, 0x02
    mov bx, disk_load_ptr
    mov al, 64
    mov dl, [boot_disk]
    mov ch, 0
    mov dh, 0x00
    mov cl, 2 ; Start from sector 2, Sector 1 is the bootstrap code 
              ; loaded from the bios 

    int 0x13
    ; Check for errors

    ; Enable the A20 line
	in al, 0x92
	or al, 2
	out 0x92, al

    ; Load the GDT
    lgdt [pm_gdt]

    ; Enable protected mode
	mov eax, cr0
	or  eax, (1 << 0)
	mov cr0, eax

    ; Do a far jump to the 32 bit entry code, because we need to flush 
    ; the CPU pipeline so it can switch to 32 bit
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

    call entry_point

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

disk_load_ptr:
incbin "build/bootloader_code.bin"
times 0x8000 - ($ - disk_load_ptr) db 0
