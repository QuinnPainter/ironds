/* First is default, others are used with -EB and -EL options */
/* Not sure why you would want big endian, so let's just stick with little. */
OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
ENTRY(__start)

MEMORY {
    /* 512k of the 4MiB EWRAM is reserved for the ARM7 */
    ewram  (wx) : ORIGIN = 0x02380000, LENGTH = 512K - 4K
    shared (w)  : ORIGIN = 0x023FF000, LENGTH = 4K
    iwram  (wx) : ORIGIN = 0x037F8000, LENGTH = 32K + 64K /* Uses 32K shared WRAM and the 64K ARM7 WRAM in one block */
}

/* 0x100 padding for BIOS Variables like irq_vec. excessive, but whatever */
__svc_stack = ORIGIN(iwram) + LENGTH(iwram) - 0x100;
__irq_stack = __svc_stack - 0x100; /* 0x100 padding for each stack */
__usr_stack = __irq_stack - 0x100;

__irq_vec   = 0x0380FFFC;
__irq_flags = 0x0380FFF8;

/* https://sourceware.org/binutils/docs/ld/PHDRS.html */
/* https://refspecs.linuxbase.org/elf/gabi4+/ch5.pheader.html#p_flags */
/* 7 flag = read, write and execute */
PHDRS {
    main PT_LOAD FLAGS(7);
    iwram PT_LOAD FLAGS(7);
}

SECTIONS {
    /* where EWRAM resident code (mostly just the init script) and any init-ed EWRAM data goes */
    .ewram : {
        *(.ewram*);
        . = ALIGN(4);
    } > ewram : main = 0xff /* this means 0xFF is used for padding, I think? */

    /* where IWRAM resident code (most code) goes */
    /* LMA = address in EWRAM, VMA = address in IWRAM */
    .text : {
        __text_lma = LOADADDR(.text);
        __text_start = ABSOLUTE(.); /* VMA */
        *(.text*);
        *(.iwram*);
        . = ALIGN(4);
        __text_end = ABSOLUTE(.);
    } > iwram AT> ewram : iwram = 0xff
    __text_size = __text_end - __text_start;

    /* read only stuff, like string literals */
    .rodata : {
        __rodata_lma = LOADADDR(.rodata);
        __rodata_start = ABSOLUTE(.); /* VMA */
        *(.rodata*);
        . = ALIGN(4);
        __rodata_end = ABSOLUTE(.);
    } > iwram AT> ewram : iwram = 0xff
    __rodata_size = __rodata_end - __rodata_start;

    /* initialised variables */
    .data : {
        __data_lma = LOADADDR(.data);
        __data_start = ABSOLUTE(.); /* VMA */
        *(.data*);
        . = ALIGN(4);
        __data_end = ABSOLUTE(.);
    } > iwram AT> ewram : iwram = 0xff
    __data_size = __data_end - __data_start;

    /* uninitialised variables */
    /* NOLOAD makes the section not loaded at runtime, so it doesn't occupy ROM */
    .bss (NOLOAD) : {
        __bss_start = ABSOLUTE(.);
        *(.bss*);
        . = ALIGN(4);
        __bss_end = ABSOLUTE(.);
    } > iwram : NONE
    __bss_size = __bss_end - __bss_start;
    __heap_start = LOADADDR(.data) + SIZEOF(.data);
    __heap_end = ORIGIN(ewram) + LENGTH(ewram);
    __heap_size = __heap_end - __heap_start;

    /* region of shared ARM9 / ARM7 memory, used for transferring data */
    /* the SORT ensures that everything should be in the same order between ARM9 / ARM7 */
    /* it is initialised by the ARM9, so no need to store it in the ARM7 binary */
    .shared (NOLOAD) : {
        *SORT(.shared*);
    } > shared : NONE

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
