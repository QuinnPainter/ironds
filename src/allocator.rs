//! Module that provides a global allocator.
//!
//! Implements memory allocation and deallocation so you can use `alloc` things,
//! like [`Vec`](mod@alloc::vec) and [`String`](alloc::string).
//!
//! NOTE: This technically doesn't actually implement the GlobalAlloc spec properly.  
//! The alignment factor given in the Layout is ignored, and all allocations are 32-bit aligned.  
//! I can't think of a reason something would need more than 32-bit alignment, so hopefully this is fine?

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

use crate::interrupt::critical_section;
use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr;

pub(crate) struct ACSLAlloc {
    free_list: *mut u8, // Pointer to the beginning of the free list. (can change)
    heap_end: *mut u8,  // pointer to the end of the heap (never changes)
    heap_size: usize,   // size of the heap in bytes (never changes)
}

// this is required for the allocator to be static
// since allocation and deallocation are in critical sections, it should be thread-safe
unsafe impl Sync for ACSLAlloc {}

unsafe impl GlobalAlloc for ACSLAlloc {
    #[cfg_attr(feature = "arm9", link_section = ".itcm.alloc")]
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

            "5:", // gmNextFree
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
                "bls    5b",            // No, keep searching

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

    #[cfg_attr(feature = "arm9", link_section = ".itcm.dealloc")]
    #[instruction_set(arm::a32)]
    #[inline(never)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        debug_assert!(!self.free_list.is_null(), "tried to deallocate before allocator init");
        critical_section!({
            asm!(
                // Check the free block list, to see what blocks we need to modify.
                // The error checks have been disabled for speed.

                "adds   r1,3",       // Round up to a multiple of 4 part 1/2
                "bcs    6f",         // If carry then len was > FFFFFFFC
                                     // and would cause a wraparound -> Err
                "bics   r1,3",       // Round up to 4-byte multiple part 2/2
                "beq    6f",         // Zero size -> Return

                "add    r1,r0",      // R1 = end of block to free
                "bcs    6f",         // An overflow here would be a tragedy.
                "mov    r2,r5",

                // Keep a delayed pointer and follow the chain
            "2:",
                "mov    r3,r2",      // R3 = Delayed pointer to cur.free blk
                "ldr    r2,[r2]",    // R2 = Current free block
                "bic    r2,1",       // Clear islarge flag
                "cmp    r1,r2",      // target blk end <= current blk start?
                "bhi    2b",         // no, keep searching
                // It's impossible under normal conditions that r1 > HeapEnd
                // therefore the check that r1 <= r2 suffices for termination,
                // no need to check for HeapEnd.

                // We need to add a block between the previous block (in R3)
                // and the current block (in R2).

                // First, check if there's a block before us that is contiguous
                // to us. If so, it needs to be extended rather than creating one.
                "cmp    r3,r5",      // is there a previous block?
                "beq    3f",         // if not, we don't need to merge it

                // Previous block present. Check if we need to add ourselves
                // to it, by checking if the end of the block = ourselves.
                // If not, we need to create a new one too.
                "ldrh   r4,[r3]",    // get previous block's islarge flag
                "tst    r4,1",       // set?
                "add    r4,r3,4",    // prepare end = start + 4
                "ldrne  r4,[r4]",    // read ptr to last if bit set

                // We now have the end of the previous block in R4; if it
                // doesn't equal the block to free, create a new one.
                "cmp    r4,r0",       // does the block end at this one?
                "moveq  r0,r3",       // move block pointer if so
                "beq    4f",          // don't create new block if so

                // Create a new head
            "3:", // fmNoMergeHead
                "ldrh   r4,[r3]",     // get previous islarge flag
                "and    r4,1",        // isolate it
                "orr    r4,r0",       // merge flag w/ initial block address
                "str    r4,[r3]",     // update last block's next ptr to point to us

            "4:", // fmCheckLast
                "mov    r5,r12",
                "cmp    r2,r5",       // if next = HeapEnd, don't merge
                "beq    5f",
                "cmp    r1,r2",       // are we touching the next block?
                "beq    7f",          // merge both if so

                // Adjust islarge in R2 and store it in [R0], and R1 in [R0+4].
            "5:", // fmNoMergeTail
                "sub    r4,r1,4",     // use r4 to not need to restore r1
                "cmp    r4,r0",       // Are we of size 4?
                "orrne  r2,1",        // If not, set islarge flag
                "str    r2,[r0]",     // Store pointer + islarge flag
                "strne  r1,[r0,4]",   // Store Last
                "b      9f",
            "6:", // fmError
                "b      9f",          // maybe this should panic instead of just exiting?

            "7:", // fmMergeTail
                "ldr    r3,[r2]",     // Grab next block's next ptr
                "tst    r3,1",        // Do the usual dance to get Last
                "orr    r3,1",        // The new islarge will surely be set
                "add    r1,r2,4",
                "ldrne  r1,[r1]",
                "str    r3,[r0]",     // Expand this block by using the next
                "str    r1,[r0,4]",   // block's Next and Last pointers
            "9:",
                in("r0") ptr,
                in("r1") layout.size(),
                inout("r5") &self.free_list as *const *mut u8 => _, // pointer to pointer
                in("r12") self.heap_end,
                out("r4") _,
                clobber_abi("C"),
                options(raw),
            );
        });
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
