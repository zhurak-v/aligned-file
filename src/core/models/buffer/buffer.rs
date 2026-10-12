pub trait Buffer {
    fn capacity(&self) -> usize;

    fn len(&self) -> usize;

    fn as_slice(&self) -> &[u8];

    fn as_capacity_slice(&self) -> &[u8];

    fn as_ptr(&self) -> *const u8;
}