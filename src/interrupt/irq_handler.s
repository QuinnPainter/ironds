    .arm
    .align 2
    .global irq_handler
    .section .iwram.irq_handler

// Run whenever an IRQ is triggered. Should not be called by user code.
// BIOS saves r0-r3, r12 and lr before calling
// https://github.com/melonDS-emu/melonDS/blob/master/freebios/bios_common.S#L1117
irq_handler:
    // Disable IME
    mov r12, #0x4000000
    str r12, [r12, #0x208]

    // Acknowledge interrupts in IF
    ldr r1, [r12, #210] // r1 = IE
    ldr r0, [r12, #214] // r0 = IF
    ands r1, r1, r0     // r1 = IE & IF (all requested IRQs)
    str r1, [r12, #214] // write (IE & IF) to IF = acknowledge all requested IRQs

    // Acknowledge interrupts for the BIOS IRQ Flags
    ldr r2, =__irq_flags // r2 = BIOS IF Addr (defined in linkerscript)
    ldr r0, [r2]        // r0 = BIOS IF
    orr r0, r0, r1      // r0 |= (IE & IF)
    str r0, [r2]        // write new BIOS IF

    ldr r0, =USER_IRQ_HANDLER
    ldr r0, [r0] // load the actual function pointer into r0
    cmp r0, #0   // is it 0 / None?
    beq 2f       // branch if so

    mrs r1, spsr
    push {r1, lr} // save SPSR_IRQ and LR_IRQ (needed for nested interrupts)
    //switch to system
    //save system LR and ime?

2:  
    // Disable IME again, just in case user code re-enabled it
    mov r12, #0x4000000
    str r12, [r12, #0x208]

    bx lr

