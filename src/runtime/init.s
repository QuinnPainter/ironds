    .global __start

    /* note - all double underscore symbols (other than __start)
       are defined in the linkerscript */

    .section .text.rt0
    .arm /* equivalent to "code 32" - sets the instruction set to ARM */
    .balign 4
__start:
    /* make sure interrupts are disabled by turning IME to 0 */
    /* r0 is used both as address and as value (only bottom bit matters for IME) */
    mov r0, #0x04000000
    str r0, [r0, #0x208]

    mov	r0, #0x12 /* Switch to IRQ Mode */
    msr	cpsr, r0
    ldr	sp, =__irq_stack /* Set IRQ stack */

    mov	r0, #0x13 /* Switch to SVC Mode */
    msr	cpsr, r0
    ldr	sp, =__svc_stack /* Set SVC stack */

    mov	r0, #0x1F /* Switch to System Mode */
    msr	cpsr, r0
    ldr	sp, =__usr_stack /* Set user / system stack */

    ldr r0, =__bss_start /* Clear BSS */
    ldr r1, =__bss_size
    bl zero_mem

    ldr r0, =lib_init
    blx r0

    /* main shouldn't return, but if it does, this lib function will run */
    ldr r0, =main
    ldr lr, =return_from_main
    bx r0 /* jump to user code */


/*  Set a block of memory to 0
    r0 = Start Address
    r1 = Length (bytes) */
zero_mem:
    add r1, r1, #3  /* round up if misaligned */
    bics r1, r1, #3 /* make sure length is aligned, clear last 2 bits */
    bxeq lr         /* quit if length is 0 */

    mov r2, #0
.lp:
    str r2, [r0], #4
    subs r1, r1, #4
    bne .lp
    bx lr

    .thumb /* switch to THUMB - is this necessary? */
    .previous /* go back to previous section */
