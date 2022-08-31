/* First is default, others are used with -EB and -EL options */
/* Not sure why you would want big endian, so let's just stick with little. */
OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
ENTRY(__start)

MEMORY {
    /* 512k of the 4MiB EWRAM is reserved for the ARM7 */
    ewram (wx) : ORIGIN = 0x02380000, LENGTH = 512k
    iwram (wx) : ORIGIN = 0x037F8000, LENGTH = 32K + 64K /* Uses 32K shared WRAM and the 64K ARM7 WRAM in one block */
}

__svc_stack = ORIGIN(iwram) + LENGTH(iwram);
__irq_stack = __svc_stack - 0x100;
__usr_stack = __irq_stack - 0x100;

/* https://sourceware.org/binutils/docs/ld/PHDRS.html */
/* https://refspecs.linuxbase.org/elf/gabi4+/ch5.pheader.html#p_flags */
/* 7 flag = read, write and execute */
PHDRS {
    main PT_LOAD FLAGS(7);
    iwram PT_LOAD FLAGS(7);
}

SECTIONS {
    /* where EWRAM resident code goes */
    .text : {
        *(.text*);
        . = ALIGN(4);
    } > ewram : main = 0xff /* this means 0xFF is used for padding, I think? */

    /* read only stuff, like string literals */
    .rodata : {
        *(.rodata*);
        . = ALIGN(4);
    } > ewram : main = 0xff

    /* initialised variables */
    .data : {
        *(.data*);
        . = ALIGN(4);
    } > ewram : main = 0xff

    /* uninitialised variables */
	/* NOLOAD makes the section not loaded at runtime, so it doesn't occupy ROM */
    .bss (NOLOAD) : {
		__bss_start = ABSOLUTE(.);
        *(.bss*);
        . = ALIGN(4);
		__bss_end = ABSOLUTE(.);
		__heap_start = ABSOLUTE(.);
    } > ewram : main
	__bss_size = __bss_end - __bss_start;
	__heap_end = ORIGIN(ewram) + LENGTH(ewram);
	__heap_size = __heap_end - __heap_start;

    /* since heap is defined before IWRAM, they will overlap. */
    /* this should be fine, as the EWRAM LMAs are no longer needed */
    /* after they have been loaded into IWRAM. */
    /* hopefully this isn't a stupid idea? */

    /* LMA = address in EWRAM, VMA = address in IWRAM */
    .iwram : {
        __iwram_lma = LOADADDR(.iwram);
        __iwram_start = ABSOLUTE(.); /* VMA */
        *(.iwram*);
        . = ALIGN(4);
        __iwram_end = ABSOLUTE(.);
    } > iwram AT> ewram : iwram = 0xff
    __iwram_size = __iwram_end - __iwram_start;

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
