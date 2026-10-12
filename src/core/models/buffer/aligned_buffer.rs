use std::alloc::{alloc, Layout, dealloc};

use crate::core::models::alignment::alignment::Alignment;
use crate::core::models::buffer::buffer::Buffer;
use crate::core::models::buffer::mutable_buffer::MutableBuffer;

pub struct AlignedBuffer {
    capacity: usize,
    len: usize,
    alignment: Alignment,
    layout: Layout,
    ptr: *mut u8,
}

impl AlignedBuffer {
    #[inline]
    pub fn allocate(capacity: usize, alignment: Alignment) -> AlignedBuffer {
        let rounded_capacity: usize = Alignment::round_up(capacity, alignment);
        let layout: Layout = Layout::from_size_align(
            rounded_capacity, alignment.as_usize()
        ).unwrap();
        AlignedBuffer {
            capacity: rounded_capacity,
            len: 0,
            alignment,
            layout,
            ptr: unsafe { alloc(layout) },
        }
    }

    #[inline]
    pub fn from_slice(src: impl AsRef<[u8]>, alignment: Alignment) -> AlignedBuffer {
        let slice: &[u8] = src.as_ref();
        let slice_len: usize = slice.len();
        let mut buffer: AlignedBuffer = AlignedBuffer::allocate(slice_len, alignment);
        let mut_capacity_slice: &mut [u8] = buffer.as_mut_capacity_slice();
        mut_capacity_slice[..slice_len].copy_from_slice(slice);
        buffer.set_len(slice_len);
        buffer
    }
}

impl Buffer for AlignedBuffer {
    #[inline]
    fn capacity(&self) -> usize {
        (*self).capacity
    }

    #[inline]
    fn len(&self) -> usize {
        (*self).len
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts((*self).ptr, (*self).len) }
    }

    #[inline]
    fn as_capacity_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts((*self).ptr, (*self).capacity) }
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        (*self).ptr as *const u8
    }
}

impl MutableBuffer for AlignedBuffer {
    #[inline]
    fn set_len(&mut self, len: usize) -> usize {
        assert!(len <= (*self).capacity());
        (*self).len = len;
        len
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut((*self).ptr, (*self).len) }
    }

    #[inline]
    fn as_mut_capacity_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut((*self).ptr, (*self).capacity) }
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        (*self).ptr
    }
}

impl Drop for AlignedBuffer {
    #[inline]
    fn drop(&mut self) {
        if (*self).capacity > 0 && !(*self).ptr.is_null() {
            unsafe { dealloc((*self).ptr, (*self).layout) }
        }
    }
}

unsafe impl Send for AlignedBuffer {}
unsafe impl Sync for AlignedBuffer {}