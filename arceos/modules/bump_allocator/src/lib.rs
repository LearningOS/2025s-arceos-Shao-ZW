#![no_std]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    b_pos: usize,
    p_pos: usize,
    end: usize,
    bytes_alloced: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            b_pos: 0,
            p_pos: 0,
            end: 0,
            bytes_alloced: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.b_pos = start;
        self.p_pos = start + size;
        self.end = (start + size) / PAGE_SIZE * PAGE_SIZE;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        todo!()
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.b_pos + layout.size() > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        let ret = self.b_pos;
        self.b_pos += layout.size();
        self.bytes_alloced += layout.size();

        // Safety:
        Ok(unsafe { NonNull::new_unchecked(ret as *mut u8) })
    }

    fn available_bytes(&self) -> usize {
        todo!()
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.bytes_alloced -= layout.size();
        if self.bytes_alloced == 0 {
            self.bytes_alloced = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        todo!()
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if self.p_pos - num_pages * PAGE_SIZE < self.b_pos {
            return Err(AllocError::NoMemory);
        }
        self.p_pos -= num_pages * PAGE_SIZE;
        Ok(self.p_pos)
    }

    fn available_pages(&self) -> usize {
        todo!()
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        todo!()
    }
}
