    .arm
    .align 2
    .global irq_handler
    .section .iwram.irq_handler

// Run whenever an IRQ is triggered. Should not be called by user code.
// BIOS saves r0-r3, r12 and lr before calling
// https://github.com/melonDS-emu/melonDS/blob/master/freebios/bios_common.S#L1117
irq_handler:
    mov r12, #0x4000000
    str r12, [r12, #0x208] // disable IME
