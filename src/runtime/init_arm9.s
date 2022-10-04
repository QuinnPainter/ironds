.set ARM7_RESERVED_EWRAM_SIZE, 0x80000 /* 512K */
.set ARM7_RESERVED_EWRAM_START, ((0x02000000 + 0x00400000) - ARM7_RESERVED_EWRAM_SIZE)

    .global __start

    /* note - many of the double underscore symbols
       are defined in the linkerscript */

    .section .text.rt0
    .arm /* equivalent to "code 32" - sets the instruction set to ARM */
    .balign 4
__start:
    /* make sure interrupts are disabled by turning IME to 0 */
    /* r0 is used both as address and as value (only bottom bit matters for IME) */
    mov r0, #0x04000000
    str r0, [r0, #0x208]

    /* Init stacks */
    mov	r0, #0x12 /* Switch to IRQ Mode */
    msr	cpsr, r0
    ldr	sp, =__irq_stack /* Set IRQ stack */

    mov	r0, #0x13 /* Switch to SVC Mode */
    msr	cpsr, r0
    ldr	sp, =__svc_stack /* Set SVC stack */

    mov	r0, #0x1F /* Switch to System Mode */
    msr	cpsr, r0
    ldr	sp, =__usr_stack /* Set user / system stack */

    /* Assign all 32K of IWRAM to the ARM7 */
    /* https://www.problemkaputt.de/gbatek.htm#dsmemorycontrolwram */
    mov r0, #3
    ldr r1, =0x04000247
    strb r0, [r1]

    /* Cache / TCM / Memory Protection Unit Init */
    /* https://www.problemkaputt.de/gbatek.htm#armcp15systemcontrolcoprocessor */
    /* https://www.problemkaputt.de/gbatek.htm#dsmemorycontrolcacheandtcm */
    ldr r0, =0x00002078 /* Disable MPU, cache and TCM */
    mcr p15, 0, r0, c1, c0, 0

    mov r0, #0
    mcr p15, 0, r0, c7, c5, 0 /* clear instruction cache */
    mcr p15, 0, r0, c7, c6, 0 /* clear data cache */ 
    mcr p15, 0, r0, c7, c10, 4 /* flush write buffer */

    ldr r0, =__dtcm_start /* DTCM base = __dtcm_start */
    orr r0, r0, (5 << 1) /* DTCM size = 16K (512 << 5) */
    mcr p15, 0, r0, c9, c1, 0

    mov r0, (16 << 1) /* ITCM size = 32M (512 << 16) (actual size is 32k, but we want mirrors) */
    mcr p15, 0, r0, c9, c1, 1 /* base is 0, ITCM will be mirrored from 0 to 0x02000000 */

    /* in the case of overlapping regions, the higher number takes priority (region 7 = max priority) */
    /* Region 0 - I/O and VRAM */
    ldr r0, =((25 << 1) | 0x04000000 | 1) /* Size = 64M (2 << 19), base = 0x04000000, enable = 1 */
    mcr p15, 0, r0, c6, c0, 0
    /* Region 1 - Main Memory */
    ldr r0, =((21 << 1) | 0x02000000 | 1) /* Size = 4M (2 << 21), base = 0x02000000, enable = 1 */
    mcr p15, 0, r0, c6, c1, 0
    /* Region 2 - ARM7 Reserved Main Memory */
    ldr r0, =((18 << 1) | ARM7_RESERVED_EWRAM_START | 1) /* Size = 512K (2 << 18), enable = 1 */
    mcr p15, 0, r0, c6, c2, 0
    /* Region 3 - GBA Slot */
    ldr r0, =((26 << 1) | 0x08000000 | 1) /* Size = 128M (2 << 26), base = 0x08000000, enable = 1 */
    mcr p15, 0, r0, c6, c3, 0
    /* Region 4 - DTCM */
    ldr r0, =__dtcm_start
    orr r0, r0, #((13 << 1) | 1) /* Size = 16K (2 << 13), base = __dtcm_start, enable = 1 */
    mcr p15, 0, r0, c6, c4, 0
    /* Region 5 - ITCM */
    ldr r0, =((14 << 1) | 0x01000000 | 1) /* Size = 32K (2 << 14), base = 0x01000000, enable = 1 */
    mcr p15, 0, r0, c6, c5, 0
    /* Region 6 - BIOS ROM */
    ldr r0, =((14 << 1) | 0xFFFF0000 | 1) /* Size = 32K (2 << 14), base = 0xFFFF0000, enable = 1 */
    mcr p15, 0, r0, c6, c6, 0
    /* Region 7 - ARM9/ARM7 Shared Region */
    /* todo - GBATEK says the minimum size is 4K, but is this true? it seems that smaller sizes should be possible. */
    ldr r0, =((11 << 1) | (0x02400000 - 4096) | 1) /* Size = 4K (2 << 11), base = (end of RAM - 4K) = 0x023FF000, enable = 1 */
    mcr p15, 0, r0, c6, c7, 0

    /* Set region attributes */
    /* Instruction / Data Cache Enable */
    mov r0, #0b01000010 /* only main memory and BIOS ROM use the caches */
    mcr p15, 0, r0, c2, c0, 0 /* data */
    mcr p15, 0, r0, c2, c0, 1 /* instruction */
    /* Write Bufferability (0 = Write-Through, 1 = Write-Back) */
    mov r0, #0b00000010 /* only main memory can buffer writes */
    mcr p15, 0, r0, c3, c0, 0
    /* Read/Write Access */
    ldr r0, =0x06333033 /* unused and arm7 reserved have no access, BIOS is read-only, rest are R/W */
    mcr p15, 0, r0, c5, c0, 2 /* data */
    mcr p15, 0, r0, c5, c0, 3 /* instruction */

    mrc p15, 0, r0, c1, c0, 0
    ldr r1, =(1 << 18) | (1 << 16) | (1 << 12) | (1 << 2) | 1
    orr r0, r0, r1 /* enable TCM, cache and MPU */
    mcr p15, 0, r0, c1, c0, 0

    /* Init the code / data regions */
    ldr r0, =__bss_start /* Clear BSS */
    ldr r1, =__bss_size
    bl __init_zero_mem

    ldr r0, =__itcm_start /* Copy ITCM from LMA to VMA */
    ldr r1, =__itcm_lma
    ldr r2, =__itcm_size
    bl __init_memcpy

    ldr r0, =__dtcm_start /* Copy DTCM from LMA to VMA */
    ldr r1, =__dtcm_lma
    ldr r2, =__dtcm_size
    bl __init_memcpy

    /* Setup IRQ vector */
    ldr r0, =__irq_vec
    ldr r1, =irq_handler
    str r1, [r0]

    ldr r0, =lib_init
    blx r0

    /* main shouldn't return, but if it does, this lib function will run */
    ldr r0, =main
    ldr lr, =return_from_main
    bx r0 /* jump to user code */


/*  Set a block of memory to 0
    r0 = Start Address (assumed to be 32 bit aligned)
    r1 = Length (bytes) */
__init_zero_mem:
    add r1, r1, #3  /* round up if misaligned */
    bics r1, r1, #3 /* make sure length is aligned, clear last 2 bits */
    bxeq lr         /* quit if length is 0 */

    mov r2, #0
3:  str r2, [r0], #4
    subs r1, r1, #4
    bne 3b
    bx lr

/* Simple bytewise memcpy (could optimise this, but it's not critical as it's only used for init)
   r0 = Dest Address
   r1 = Source Address
   r2 = Length (bytes) */
__init_memcpy:
    cmp r2, #0
    bxeq lr /* quit if length is 0 */

2:  ldrb r3, [r1], #1
    strb r3, [r0], #1
    subs r2, r2, #1
    bne 2b
    bx lr
