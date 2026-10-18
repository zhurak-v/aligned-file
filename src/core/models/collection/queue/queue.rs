use crate::core::models::collection::collection::Collection;

pub trait Queue<T>: Collection<T> {

    fn offer(&mut self, item: T) -> bool;

    fn poll(&mut self) -> Option<T>;

    fn peek(&self) -> Option<&T>;

}