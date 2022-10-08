    .global __start

    /* note - many of the double underscore symbols are defined in the linkerscript */
    /* ALSO: most of this is running from EWRAM. Since the ARM7 has EWRAM priority, the ARM9
       is pretty much locked up while this is running. Any part requiring ARM9 synchronisation
       should be copied into IWRAM first. */

    .section .ewram.rt0, "ax"
    .arm /* equivalent to "code 32" - sets the instruction set to ARM */
    .balign 4
__start:
    mov r11, r11
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

    /* Copy wait for RAM loop into IWRAM (the 64K part, since we don't have access to Shared WRAM yet!) */
    mov r0, 0x03800000   /* start of IWRAM - this will be overwritten later, but that's fine */
    mov r12, r0
    ldr r1, =__init_wait_for_ram
    ldr r2, =__init_wait_for_ram_end-__init_wait_for_ram
    bl __init_memcpy
    adr lr, 2f
    bx r12 /* call __init_wait_for_ram in IWRAM */
2:

    /* Init the code / data regions */
    ldr r0, =__bss_start /* Clear BSS */
    ldr r1, =__bss_size
    bl __init_zero_mem

    /* todo: could probably combine all these into a single memcpy */
    ldr r0, =__text_start /* Copy text section from LMA to VMA */
    ldr r1, =__text_lma
    ldr r2, =__text_size
    bl __init_memcpy

    ldr r0, =__rodata_start /* Copy rodata section from LMA to VMA */
    ldr r1, =__rodata_lma
    ldr r2, =__rodata_size
    bl __init_memcpy

    ldr r0, =__data_start /* Copy data section from LMA to VMA */
    ldr r1, =__data_lma
    ldr r2, =__data_size
    bl __init_memcpy

    /* Setup IRQ vector */
    ldr r0, =__irq_vec
    ldr r1, =irq_handler
    str r1, [r0]

    ldr r0, =lib_init
    bl 5f /* bl to "bx r0" */

    /* main shouldn't return, but if it does, this lib function will run */
    ldr r0, =main
    ldr lr, =return_from_main
5:  bx r0 /* jump to user code */

__init_wait_for_ram:
    /* Wait until the ARM9 has assigned the shared RAM to the ARM7 */
    /* https://www.problemkaputt.de/gbatek.htm#dsmemorycontrolwram */
    ldr r1, =0x04000241
4:  ldrb r0, [r1]
    and r0, r0, #3
    cmp r0, #3
    bne 4b
    bx lr
__init_wait_for_ram_end:

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
