/* First is default, others are used with -EB and -EL options */
/* Not sure why you would want big endian, so let's just stick with little. */
OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
ENTRY(__start)

MEMORY {
    /* 512k reserved for the ARM7 */
    ewram (wx) : ORIGIN = 0x02000000, LENGTH = 4M - 512k
    dtcm  (w)  : ORIGIN = 0x02ff0000, LENGTH = 16K
    itcm  (wx) : ORIGIN = 0x01000000, LENGTH = 32K
}

SECTIONS {
    /* where code goes */
    .text : {
        *(.text*);
        . = ALIGN(4);
    } > ewram = 0xff /* this means 0xFF is used for padding, I think? */

    /* read only stuff, like string literals */
    .rodata : {
        *(.rodata*);
        . = ALIGN(4);
    } > ewram = 0xff

    /* initialised variables */
    .data : {
        *(.data*)
        . = ALIGN(4);
    } > ewram = 0xff

    /* uninitialised variables */
    .bss : {
        *(.bss*)
        . = ALIGN(4);
    } > ewram

    /* debugging sections */
    /* Stabs */
    .stab            0 : { *(.stab) }
    .stabstr         0 : { *(.stabstr) }
    .stab.excl       0 : { *(.stab.excl) }
    .stab.exclstr    0 : { *(.stab.exclstr) }
    .stab.index      0 : { *(.stab.index) }
    .stab.indexstr   0 : { *(.stab.indexstr) }
    .comment         0 : { *(.comment) }
    /* DWARF 1 */
    .debug           0 : { *(.debug) }
    .line            0 : { *(.line) }
    /* GNU DWARF 1 extensions */
    .debug_srcinfo   0 : { *(.debug_srcinfo) }
    .debug_sfnames   0 : { *(.debug_sfnames) }
    /* DWARF 1.1 and DWARF 2 */
    .debug_aranges   0 : { *(.debug_aranges) }
    .debug_pubnames  0 : { *(.debug_pubnames) }
    /* DWARF 2 */
    .debug_info      0 : { *(.debug_info) }
    .debug_abbrev    0 : { *(.debug_abbrev) }
    .debug_line      0 : { *(.debug_line) }
    .debug_frame     0 : { *(.debug_frame) }
    .debug_str       0 : { *(.debug_str) }
    .debug_loc       0 : { *(.debug_loc) }
    .debug_macinfo   0 : { *(.debug_macinfo) }
    /* SGI/MIPS DWARF 2 extensions */
    .debug_weaknames 0 : { *(.debug_weaknames) }
    .debug_funcnames 0 : { *(.debug_funcnames) }
    .debug_typenames 0 : { *(.debug_typenames) }
    .debug_varnames  0 : { *(.debug_varnames) }
}
