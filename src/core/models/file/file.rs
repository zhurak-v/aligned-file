use std::path::Path;

pub trait File {
    fn path(&self) -> &Path;
}