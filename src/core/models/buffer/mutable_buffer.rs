use crate::core::models::buffer::buffer::Buffer;

pub trait MutableBuffer: Buffer {
    fn set_len(&mut self, len: usize) -> usize;
    
    fn as_mut_slice(&mut self) -> &mut [u8];
    
    fn as_mut_capacity_slice(&mut self) -> &mut [u8];
    
    fn as_mut_ptr(&mut self) -> *mut u8;
}