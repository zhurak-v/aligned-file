pub trait Collection<T>: IntoIterator<Item = T> {

    fn size(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        (*self).size() == 0
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>
    where
        T: 'a;

    #[inline]
    fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        (*self).iter().any(|x| x == item)
    }

    fn add(&mut self, item: T) -> bool;

    fn remove(&mut self, item: &T) -> bool
    where
        T: PartialEq;

    fn clear(&mut self);

    #[inline]
    fn add_all<I>(&mut self, items: I) -> bool
    where
        I: IntoIterator<Item = T>,
        Self: Sized,
    {
        let mut changed: bool = false;
        for item in items {
            changed |= (*self).add(item);
        }
        changed
    }

    #[inline]
    fn contains_all<'b, I>(&self, items: I) -> bool
    where
        I: IntoIterator<Item = &'b T>,
        T: PartialEq + 'b,
    {
        items.into_iter().all(|x| (*self).contains(x))
    }

}