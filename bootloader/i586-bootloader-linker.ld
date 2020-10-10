ENTRY(entry)

SECTIONS {
    . = 0x7e00;

    .start : 
    {
        *(.start)
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata : 
    {
        *(.rodata .rodata.*)
    }

    .data.rel.ro : 
    {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}
