/* First is default, others are used with -EB and -EL options */
/* Not sure why you would want big endian, so let's just stick with little. */
OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
ENTRY(__start)

MEMORY {
    /* 512k reserved for the ARM7 */
    ewram  (wx) : ORIGIN = 0x02000000, LENGTH = 4M - 512k
    shared (w)  : ORIGIN = 0x023FF000, LENGTH = 4K
    itcm   (wx) : ORIGIN = 0x01000000, LENGTH = 32K
    dtcm   (w)  : ORIGIN = 0x02ff0000, LENGTH = 16K
}

/* 0x100 padding for BIOS Variables like irq_vec. excessive, but whatever */
__svc_stack = ORIGIN(dtcm) + LENGTH(dtcm) - 0x100;
__irq_stack = __svc_stack - 0x100; /* 0x100 padding for each stack */
__usr_stack = __irq_stack - 0x100;

__irq_vec = (ORIGIN(dtcm) + LENGTH(dtcm)) - 4;
__irq_flags = (ORIGIN(dtcm) + LENGTH(dtcm)) - 8;

/* https://sourceware.org/binutils/docs/ld/PHDRS.html */
/* https://refspecs.linuxbase.org/elf/gabi4+/ch5.pheader.html#p_flags */
/* 7 flag = read, write and execute */
/* use same segment names as devkitarm, to (hopefully) be compatible with ndstool */
PHDRS {
    main PT_LOAD FLAGS(7);
    itcm PT_LOAD FLAGS(7);
    dtcm PT_LOAD FLAGS(7);
    shared PT_LOAD FLAGS(6); /* not executable */
}

SECTIONS {
	/* Secure area. usually 0 filled for 2048 bytes */
	/* https://problemkaputt.de/gbatek.htm#dscartridgesecurearea */
	.secure : {
		KEEP(*(.secure));
	} > ewram : main = 0x00

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
    /* the decision to lay things out this way means this is only compatible with ndstool pre-2.2.0 */
    /* https://github.com/devkitPro/ndstool/commit/21bbea5731cbcf6f9b46245646f28f910db7a74a */
    /* since it was changed to not allow "holes" between PHDR sections */
    /* so bss would need to be moved to the end, which would mean losing the heap overlap optimisation */
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

    /* since heap is defined before the following sections, they will overlap. */
    /* this should be fine, as the EWRAM LMAs are no longer needed */
    /* after they have been loaded into ITCM, DTCM and SHARED. */
    /* hopefully this isn't a stupid idea? */

    /* stuff loaded into ITCM */
    /* LMA = address in EWRAM, VMA = address in ITCM */
    .itcm : {
        __itcm_lma = LOADADDR(.itcm);
        __itcm_start = ABSOLUTE(.); /* VMA */
        *(.itcm*);
        *(.iwram*); /* put iwram stuff in ITCM, so code can be the same between ARM9/ARM7 */
        . = ALIGN(4);
        __itcm_end = ABSOLUTE(.);
    } > itcm AT> ewram : itcm = 0xff
    __itcm_size = __itcm_end - __itcm_start;

    .dtcm : {
        __dtcm_lma = LOADADDR(.dtcm);
        __dtcm_start = ABSOLUTE(.); /* VMA */
        *(.dtcm*);
        . = ALIGN(4);
        __dtcm_end = ABSOLUTE(.);
    } > dtcm AT> ewram : dtcm = 0xff
    __dtcm_size = __dtcm_end - __dtcm_start;

    /* region of shared ARM9 / ARM7 memory, used for transferring data */
    /* the SORT ensures that everything should be in the same order between ARM9 / ARM7 */
    .shared : {
        __shared_lma = LOADADDR(.shared);
        __shared_start = ABSOLUTE(.); /* VMA */
        *SORT(.shared*);
        . = ALIGN(4);
        __shared_end = ABSOLUTE(.);
    } > shared AT> ewram : shared = 0xff
    __shared_size = __shared_end - __shared_start;

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
