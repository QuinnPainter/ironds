use core::{ptr::NonNull, alloc::Layout};

// A chunk of data placed at the start of every free block of memory.
// NonNull is equivalent to a mutable pointer, with the guarantee that it cannot be null.
struct Hole {
    pub size: usize,
    pub next: Option<NonNull<Hole>>,
}

// this only needs to be sizeof(Hole), but making it a bit bigger might prevent tiny holes from clogging things up?
const MIN_ALLOC_SIZE: usize = 64;

struct Scanner {
    req_size: usize,
    req_align: usize,
    cur_hole: NonNull<Hole>,
}

impl Scanner {
    pub fn test_fit(&self) -> Result<(), ()> {
        let cur_hole_size = unsafe { self.cur_hole.as_ref().size };
        if cur_hole_size < self.req_size {
            return Err(());
        }

        let hole_ptr = self.cur_hole.as_ptr() as *mut u8;
        let req_offset = hole_ptr.align_offset(self.req_align);
        if req_offset > 0 {
            // Since we're opening up some space with our alignment offset, we need enough space for a full new hole
            let new_req_offset = hole_ptr.wrapping_add(MIN_ALLOC_SIZE).align_offset(self.req_align);
            if cur_hole_size < new_req_offset + MIN_ALLOC_SIZE + self.req_size {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn goto_next(&mut self) -> Result<(), ()> {
        unsafe{ self.cur_hole = self.cur_hole.as_ref().next.ok_or(())?; }
        Ok(())
    }
}

pub struct FLAlloc {
    firsthole: Option<NonNull<Hole>>,
}

impl FLAlloc {
    // This should only be called from lib init (where interrupts are disabled)
    // so a critical section is not necessary.
    pub unsafe fn init (&mut self, heap_start: *mut u8, heap_size: usize) {
        (heap_start as *mut Hole).write(Hole {
            size: heap_size,
            next: None,
        });
        self.firsthole = Some(NonNull::new_unchecked(heap_start as *mut Hole));
    }

    /*fn try_allocate(&mut self, hole: &Option<NonNull<Hole>>, layout: Layout) -> Result<*mut u8, ()> {
        let hole_ptr = hole.ok_or(())?.as_ptr();
        let hole = unsafe { hole.ok_or(())?.as_ref() };

        if hole.size < layout.size() {
            return Err(());
        }

        return Ok(hole_ptr as *mut u8);
    }*/

    pub fn allocate_first_fit(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let actual_size = layout.size().max(MIN_ALLOC_SIZE);

        let mut scanner = Scanner {
            req_size: actual_size,
            req_align: layout.align(),
            cur_hole: self.firsthole.ok_or(())?,
        };

        loop {
            if scanner.test_fit().is_ok() {
                //allocate
                //return
            }

            scanner.goto_next()?;
        }
    }
}
