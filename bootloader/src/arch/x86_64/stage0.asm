[org  0x7c00]
[bits 16]

%define MEMORY_MAP_ADDRESS 0x0500

; Support 'modern standard MBR'  

entry:
    cli
    cld

    mov sp, 0x7c00

    mov [boot_disk], dl

    ; Load some space
    mov ah, 0x02
    mov bx, disk_load_ptr
    mov al, 128
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

    call build_memory_map

    ; Load the GDT
    lgdt [pm_gdt]

    ; Enable protected mode
	mov eax, cr0
	or  eax, (1 << 0)
	mov cr0, eax

    ; Do a far jump to the 32 bit entry code, because we need to flush 
    ; the CPU pipeline so it can switch to 32 bit
    jmp 0x0008:pm_entry

; 'si' is the address of the message
; Original Code from https://github.com/pdoane/osdev
print:
        push ax
        mov ah, 0x0e
.loop:
        lodsb
        cmp al, 0
        je .done
        int 0x10
        jmp .loop
.done:
        pop ax
        ret

; Original Code from https://github.com/pdoane/osdev
build_memory_map:
        mov di, MEMORY_MAP_ADDRESS  ; Destination for memory map storage
        xor ebx, ebx                ; State for BIOS call, set to 0 initially

.loop:
        mov eax, 0xe820             ; Call int 0x15, 0xe820 memory map
        mov edx, 0x534D4150
        mov ecx, 24
        int 0x15

        jc .done                    ; Carry means unsupported or end of list

        cmp eax, 0x534D4150         ; EAX should match EDX
        jne .done

        jcxz .next_entry            ; Skip zero-length entries

        cmp cl, 20                  ; Test for ACPI 3.X entry
        jbe .good_entry

        test byte [es:di + 20], 1   ; Test ACPI 3.X ignore bit
        je .next_entry

.good_entry:
        add di, 24                  ; Found a valid entry

.next_entry:
        test ebx, ebx               ; Go to next entry
        jne .loop

.done:
        xor ax, ax                  ; Write terminating entry
        mov cx, 12
        rep stosw
        ret

[bits 32]

pm_entry:
    mov ax, 0x10
    mov es, ax
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov ebp, 0x7c00
    mov esp, ebp

    call entry_point

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
times 0x10000 - ($ - disk_load_ptr) db 0
