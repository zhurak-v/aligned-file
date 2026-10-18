use crate::core::models::collection::queue::queue::Queue;
use crate::core::models::comparator::comparator::Comparator;

pub trait SortedQueue<T>: Queue<T> {

    fn comparator(&self) -> &dyn Comparator<T>;

    fn lower(&self, item: &T) -> Option<&T>;

    fn lower_or_equal(&self, item: &T) -> Option<&T>;

    fn higher(&self, item: &T) -> Option<&T>;

    fn higher_or_equal(&self, item: &T) -> Option<&T>;

}