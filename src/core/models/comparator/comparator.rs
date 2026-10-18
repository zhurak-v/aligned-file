use std::cmp::Ordering;

pub trait Comparator<T> {
    fn compare(&self, a: &T, b: &T) -> Ordering;

    fn reversed(&self) -> impl Comparator<T>
    where
        Self: Sized,
    {
        ReversedComparator { comparator: self }
    }
}

struct ReversedComparator<'a, C> {
    comparator: &'a C,
}

impl<T, C: Comparator<T>> Comparator<T> for ReversedComparator<'_, C> {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        (*self).comparator.compare(a, b).reverse()
    }
}