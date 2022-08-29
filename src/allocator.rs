// Based on the malloc implementation in ACSL: (copied as of 2022-08-27)
// https://codeberg.org/pgimeno/ACSL/src/branch/master/stdlib/malloc_free.s
// used under the ISC License:
// (C) Copyright 2021 Pedro Gimeno Fortea
// Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is
// hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE
// FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
// ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

// NOTE: This technically doesn't actually implement the GlobalAlloc spec properly:
// the alignment factor given in the Layout is ignored, and all allocations are 32-bit aligned.
// can't think of a reason something would need more than 32-bit alignment, so hopefully this is fine?

use core::arch::asm;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use crate::interrupt::critical_section;

pub struct ACSLAlloc {
    free_list: *mut u8, // Pointer to the beginning of the free list. (can change)
    heap_end: *mut u8, // pointer to the end of the heap (never changes)
    heap_size: usize, // size of the heap in bytes (never changes)
}

// this is required for the allocator to be static
// since allocation and deallocation are in critical sections, it should be thread-safe
unsafe impl Sync for ACSLAlloc {}

unsafe impl GlobalAlloc for ACSLAlloc {
    #[link_section = ".itcm"]
    #[instruction_set(arm::a32)]
    #[inline(never)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        debug_assert!(!self.free_list.is_null(), "tried to allocate before allocator init");
        debug_assert!(layout.align() <= 4,
            "greater than 4 alignment for allocation is not supported (requested alignment={})", layout.align());
        let allocated_addr: usize;
        critical_section!({
            asm!(
                // r0 = size of block
                // Note: malloc will always pass r0 > 4.

                "adds   r0,3",      // Round up to the next multiple of 4
                "bcs    2f",        // If carry then len was > FFFFFFFC
                                    // and would cause a wraparound -> Err
                "bic    r0,3",      // 4-byte alignment (granularity)

                // R3 = max size
                "cmp    r0,r3",     // Error if size > max size
                "bhi    2f",        // (avoids overflows in calculations)

                "mov    r4,r5",     // previous pointer in r4
                "ldr    r5,[r5]",
                "cmp    r5,r12",    // end of memory?
                "bhs    2f",

            "1:", // gmNextFree
                "add    r2,r5,4",       // R2 = ^Last or place where ^Last is
                "ldr    r1,[r5]",       // R1 = pointer to next free block
                "tst    r1,1",          // Bit 0: islarge flag
                "bicne  r1,1",          // Clear it so it's a valid pointer
                "ldrne  r2,[r2]",       // Grab ^Last from this address

                // r1 = address of next free block
                // r2 = address of end of current block
                "add    r3,r5,r0",      // candidate ptr + alloc size
                "cmp    r3,r2",         // Is there room in this block?
                "bls    3f",            // If so, jump to arrange stuff
                "mov    r4,r5",         // R4 = previous block pointer
                "mov    r5,r1",         // R5 = pointer to new block
                "cmp    r5,r12",        // Last block?
                "bls    1b",            // No, keep searching

            "2:", // gmError
                "subs   r0,r0",     // Return null pointer and CF=1
                "b 9f",

            "3:", // gmFoundRoom
                "beq    4f",        // Fits exactly; this block disappears
            
                // Shrink block; R3 holds where the shrunk free block should be
                "sub    r12,r2,4",  // used to check if block length is 4
                "cmp    r3,r12",    // is it?
                "orrne  r1,1",      // set flag if not
                "strne  r2,[r3,4]", // store the end of block if not
                "str    r1,[r3]",   // store pointer to next + islarge flag
                "ldrh   r1,[r4]",
                "and    r1,1",
                "orr    r1,r3",
                "str    r1,[r4]",   // update pointer to previous block
                "adds   r0,r5,0",   // Return allocated pointer and CF=0
                "b 9f",

            "4:", // gmVanishBlock
                // Remove this block.
            
                "ldrh   r0,[r4]",   // Get flag in bit 0 (reusing R0 here)
                "and    r0,1",      // Isolate it
                "bic    r1,1",      // Clear flag in pointer to next
                "orr    r1,r0",     // Copy flag from previous pointer
                "str    r1,[r4]",   // Overwrite previous pointer
                "adds   r0,r5,0",   // Return allocated pointer and CF=0",
            "9:",
                inout("r0") layout.size() => allocated_addr,
                inout("r5") &self.free_list as *const *mut u8 => _, // pointer to pointer
                in("r3") self.heap_size,
                in("r12") self.heap_end,
                out("r4") _,
                clobber_abi("C"),
                options(raw),
            );
        });
        allocated_addr as *mut u8
    }

    #[link_section = ".itcm"]
    #[instruction_set(arm::a32)]
    #[inline(never)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {

    }
}

impl ACSLAlloc {
    pub const fn new() -> ACSLAlloc {
        ACSLAlloc { free_list: ptr::null_mut(), heap_end: ptr::null_mut(), heap_size: 0 }
    }

    pub fn init(&mut self, heap_start: *mut u8, heap_size: usize) {
        // We assume that HeapEnd - HeapOrg >= 8.
        self.free_list = heap_start;
        self.heap_size = heap_size;
        self.heap_end = heap_start.wrapping_add(heap_size);
        unsafe {
            ptr::write(heap_start as *mut usize, self.heap_end.wrapping_add(1) as usize);
            ptr::write(heap_start.wrapping_add(4) as *mut usize, self.heap_end as usize);
        }
    }
}
