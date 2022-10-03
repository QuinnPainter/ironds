    .arm
    .align 2
    .global irq_handler
    .section .iwram.irq_handler

// Run whenever an IRQ is triggered. Should not be called by user code.
// BIOS saves r0-r3, r12 and lr before calling
// https://github.com/melonDS-emu/melonDS/blob/master/freebios/bios_common.S#L1117
irq_handler:
    // Disable IME, and save its previous value into R3
    mov r12, #0x4000000
    add r0, r12, #0x208
    swp r3, r12, [r0] // only bottom bit of r12 (0) matters

    // Acknowledge interrupts in IF
    ldr r1, [r12, #0x210] // r1 = IE
    ldr r0, [r12, #0x214] // r0 = IF
    and r0, r0, r1     // r0 = IE & IF (all requested IRQs)
    str r0, [r12, #0x214] // write (IE & IF) to IF = acknowledge all requested IRQs

    // Acknowledge interrupts for the BIOS IRQ Flags
    ldr r2, =__irq_flags // r2 = BIOS IF Addr (defined in linkerscript)
    ldr r1, [r2]        // r1 = BIOS IF
    orr r1, r1, r0      // r1 |= (IE & IF)
    str r1, [r2]        // write new BIOS IF

    ldr r1, =USER_IRQ_HANDLER
    ldr r1, [r1] // load the actual function pointer into r0
    cmp r1, #0   // is it 0 / None?
    beq 3f       // branch if so

    mrs r2, spsr
    push {r2, r3, lr} // save SPSR_IRQ, OLD_IME and LR_IRQ (IRQ SPSR and LR needed for nested interrupts)
    
    mrs r2, cpsr
    mov r3, r2 // save IRQ CPSR in r3
    bic r2, r2, #0xDF // Enable IRQs (and FIQ, because why not)
    orr r2, r2, #0x1F // Switch to System mode
    msr cpsr, r2

    push {r3, lr} // save IRQ CPSR and System LR to System stack

    // TODO: the AAPCS ABI technically says the stack should be 64 bit aligned here. maybe should do that?
    // r0 = (IE & IF), passed into the user function
    adr lr, 2f
    bx r1 // jump to user handler
2:
    // Disable IME while we mess with stuff
    mov r12, #0x4000000
    str r12, [r12, #0x208]

    pop {r2, lr} // restore IRQ CPSR and System LR from System stack
    msr cpsr, r2 // go back to IRQ mode

    pop {r1, r3, lr} // restore SPSR_IRQ, OLD_IME and LR_IRQ from IRQ stack
    msr spsr, r1
3:
    // Restore IME
    mov r12, #0x4000000
    str r3, [r12, #0x208]

    bx lr
