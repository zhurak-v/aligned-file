use std::cmp::Ordering;

use crate::core::models::collection::collection::Collection;
use crate::core::models::collection::queue::queue::Queue;
use crate::core::models::collection::queue::sorted_queue::SortedQueue;
use crate::core::models::comparator::comparator::Comparator;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    height: i32,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {

    #[inline]
    fn new(item: T) -> Box<Node<T>> {
        Box::new(Node {
            item,
            height: 1,
            left: None,
            right: None,
        })
    }

    #[inline]
    fn height_of(link: &Link<T>) -> i32 {
        match *link {
            Some(ref node) => (**node).height,
            None => 0,
        }
    }

    #[inline]
    fn update(&mut self) {
        let left_height: i32 = Node::height_of(&(*self).left);
        let right_height: i32 = Node::height_of(&(*self).right);
        (*self).height = 1 + left_height.max(right_height);
    }

    #[inline]
    fn balance_factor(&self) -> i32 {
        let left_height: i32 = Node::height_of(&(*self).left);
        let right_height: i32 = Node::height_of(&(*self).right);
        left_height - right_height
    }

    #[inline]
    fn rotate_right(mut self: Box<Node<T>>) -> Box<Node<T>> {
        let mut left: Box<Node<T>> = (*self).left.take().unwrap();
        (*self).left = (*left).right.take();
        (*self).update();
        (*left).right = Some(self);
        (*left).update();
        left
    }

    #[inline]
    fn rotate_left(mut self: Box<Node<T>>) -> Box<Node<T>> {
        let mut right: Box<Node<T>> = (*self).right.take().unwrap();
        (*self).right = (*right).left.take();
        (*self).update();
        (*right).left = Some(self);
        (*right).update();
        right
    }

    #[inline]
    fn rebalance(mut self: Box<Node<T>>) -> Box<Node<T>> {
        (*self).update();
        let balance: i32 = (*self).balance_factor();

        if balance > 1 {
            let left_ref: &Box<Node<T>> = (*self).left.as_ref().unwrap();
            let left_balance: i32 = (**left_ref).balance_factor();
            if left_balance < 0 {
                let left: Box<Node<T>> = (*self).left.take().unwrap();
                (*self).left = Some(left.rotate_left());
            }
            return self.rotate_right();
        }

        if balance < -1 {
            let right_ref: &Box<Node<T>> = (*self).right.as_ref().unwrap();
            let right_balance: i32 = (**right_ref).balance_factor();
            if right_balance > 0 {
                let right: Box<Node<T>> = (*self).right.take().unwrap();
                (*self).right = Some(right.rotate_right());
            }
            return self.rotate_left();
        }

        self
    }

    #[inline]
    fn insert(link: Link<T>, item: T, comparator: &dyn Comparator<T>) -> Box<Node<T>> {
        let mut node: Box<Node<T>> = match link {
            Some(node) => node,
            None => return Node::new(item),
        };

        let ordering: Ordering = (*comparator).compare(&item, &(*node).item);

        if ordering == Ordering::Less {
            let left: Link<T> = (*node).left.take();
            (*node).left = Some(Node::insert(left, item, comparator));
        } else {
            let right: Link<T> = (*node).right.take();
            (*node).right = Some(Node::insert(right, item, comparator));
        }

        node.rebalance()
    }

    #[inline]
    fn pop_min(mut self: Box<Node<T>>) -> (Link<T>, T) {
        let left: Link<T> = (*self).left.take();

        if left.is_none() {
            let right: Link<T> = (*self).right.take();
            return (right, (*self).item);
        }

        let (new_left, item): (Link<T>, T) = left.unwrap().pop_min();
        (*self).left = new_left;
        (Some(self.rebalance()), item)
    }

    #[inline]
    fn collect_refs<'a>(link: &'a Link<T>, out: &mut Vec<&'a T>) {
        let node: &'a Box<Node<T>> = match *link {
            Some(ref node) => node,
            None => return,
        };

        Node::collect_refs(&(**node).left, &mut *out);
        (*out).push(&(**node).item);
        Node::collect_refs(&(**node).right, &mut *out);
    }

    #[inline]
    fn collect_owned(link: Link<T>, out: &mut Vec<T>) {
        let node: Box<Node<T>> = match link {
            Some(node) => node,
            None => return,
        };

        let node: Node<T> = *node;
        let item: T = node.item;
        let left: Link<T> = node.left;
        let right: Link<T> = node.right;

        Node::collect_owned(left, &mut *out);
        (*out).push(item);
        Node::collect_owned(right, &mut *out);
    }

}

impl<T: PartialEq> Node<T> {

    #[inline]
    fn remove(link: Link<T>, item: &T, comparator: &dyn Comparator<T>) -> (Link<T>, bool) {
        let mut node: Box<Node<T>> = match link {
            Some(node) => node,
            None => return (None, false),
        };

        let ordering: Ordering = (*comparator).compare(item, &(*node).item);
        let removed: bool;

        if ordering == Ordering::Less {
            let left: Link<T> = (*node).left.take();
            let (new_left, found): (Link<T>, bool) = Node::remove(left, item, comparator);
            (*node).left = new_left;
            removed = found;
        } else if ordering == Ordering::Greater {
            let right: Link<T> = (*node).right.take();
            let (new_right, found): (Link<T>, bool) = Node::remove(right, item, comparator);
            (*node).right = new_right;
            removed = found;
        } else if (*node).item == *item {
            let left: Link<T> = (*node).left.take();
            let right: Link<T> = (*node).right.take();

            if right.is_none() {
                return (left, true);
            }
            if left.is_none() {
                return (right, true);
            }

            let (new_right, min): (Link<T>, T) = right.unwrap().pop_min();
            let mut successor: Box<Node<T>> = Node::new(min);
            (*successor).left = left;
            (*successor).right = new_right;
            return (Some(successor.rebalance()), true);
        } else {
            let right: Link<T> = (*node).right.take();
            let (new_right, found_right): (Link<T>, bool) = Node::remove(right, item, comparator);
            (*node).right = new_right;

            if found_right {
                removed = true;
            } else {
                let left: Link<T> = (*node).left.take();
                let (new_left, found_left): (Link<T>, bool) = Node::remove(left, item, comparator);
                (*node).left = new_left;
                removed = found_left;
            }
        }

        (Some(node.rebalance()), removed)
    }

}

pub struct TreeQueue<T> {
    root: Link<T>,
    size: usize,
    comparator: Box<dyn Comparator<T>>,
}

impl<T> TreeQueue<T> {

    #[inline]
    pub fn new<C>(comparator: C) -> TreeQueue<T>
    where
        C: Comparator<T> + 'static,
    {
        TreeQueue {
            root: None,
            size: 0,
            comparator: Box::new(comparator),
        }
    }

}

impl<T> IntoIterator for TreeQueue<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let size: usize = self.size;
        let mut items: Vec<T> = Vec::with_capacity(size);
        Node::collect_owned(self.root, &mut items);
        items.into_iter()
    }
}

impl<T> Collection<T> for TreeQueue<T> {

    #[inline]
    fn size(&self) -> usize {
        (*self).size
    }

    #[inline]
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>
    where
        T: 'a,
    {
        let mut items: Vec<&'a T> = Vec::with_capacity((*self).size);
        Node::collect_refs(&(*self).root, &mut items);
        Box::new(items.into_iter())
    }

    #[inline]
    fn add(&mut self, item: T) -> bool {
        (*self).offer(item)
    }

    #[inline]
    fn remove(&mut self, item: &T) -> bool
    where
        T: PartialEq,
    {
        let root: Link<T> = (*self).root.take();
        let comparator: &dyn Comparator<T> = &*(*self).comparator;
        let (new_root, removed): (Link<T>, bool) = Node::remove(root, item, comparator);
        (*self).root = new_root;

        if removed {
            (*self).size -= 1;
        }

        removed
    }

    #[inline]
    fn clear(&mut self) {
        (*self).root = None;
        (*self).size = 0;
    }

}

impl<T> Queue<T> for TreeQueue<T> {

    #[inline]
    fn offer(&mut self, item: T) -> bool {
        let root: Link<T> = (*self).root.take();
        let comparator: &dyn Comparator<T> = &*(*self).comparator;
        let new_root: Box<Node<T>> = Node::insert(root, item, comparator);
        (*self).root = Some(new_root);
        (*self).size += 1;
        true
    }

    #[inline]
    fn poll(&mut self) -> Option<T> {
        let root: Box<Node<T>> = (*self).root.take()?;
        let (new_root, item): (Link<T>, T) = root.pop_min();
        (*self).root = new_root;
        (*self).size -= 1;
        Some(item)
    }

    #[inline]
    fn peek(&self) -> Option<&T> {
        let mut current: &Link<T> = &(*self).root;
        let mut result: Option<&T> = None;

        loop {
            let node: &Box<Node<T>> = match *current {
                Some(ref node) => node,
                None => break,
            };

            result = Some(&(**node).item);
            current = &(**node).left;
        }

        result
    }

}

impl<T> SortedQueue<T> for TreeQueue<T> {

    #[inline]
    fn comparator(&self) -> &dyn Comparator<T> {
        &*(*self).comparator
    }

    #[inline]
    fn lower(&self, item: &T) -> Option<&T> {
        let mut current: &Link<T> = &(*self).root;
        let mut result: Option<&T> = None;

        loop {
            let node: &Box<Node<T>> = match *current {
                Some(ref node) => node,
                None => break,
            };

            let ordering: Ordering = (*(*self).comparator).compare(&(**node).item, item);
            if ordering == Ordering::Less {
                result = Some(&(**node).item);
                current = &(**node).right;
            } else {
                current = &(**node).left;
            }
        }

        result
    }

    #[inline]
    fn lower_or_equal(&self, item: &T) -> Option<&T> {
        let mut current: &Link<T> = &(*self).root;
        let mut result: Option<&T> = None;

        loop {
            let node: &Box<Node<T>> = match *current {
                Some(ref node) => node,
                None => break,
            };

            let ordering: Ordering = (*(*self).comparator).compare(&(**node).item, item);
            if ordering != Ordering::Greater {
                result = Some(&(**node).item);
                current = &(**node).right;
            } else {
                current = &(**node).left;
            }
        }

        result
    }

    #[inline]
    fn higher(&self, item: &T) -> Option<&T> {
        let mut current: &Link<T> = &(*self).root;
        let mut result: Option<&T> = None;

        loop {
            let node: &Box<Node<T>> = match *current {
                Some(ref node) => node,
                None => break,
            };

            let ordering: Ordering = (*(*self).comparator).compare(&(**node).item, item);
            if ordering == Ordering::Greater {
                result = Some(&(**node).item);
                current = &(**node).left;
            } else {
                current = &(**node).right;
            }
        }

        result
    }

    #[inline]
    fn higher_or_equal(&self, item: &T) -> Option<&T> {
        let mut current: &Link<T> = &(*self).root;
        let mut result: Option<&T> = None;

        loop {
            let node: &Box<Node<T>> = match *current {
                Some(ref node) => node,
                None => break,
            };

            let ordering: Ordering = (*(*self).comparator).compare(&(**node).item, item);
            if ordering != Ordering::Less {
                result = Some(&(**node).item);
                current = &(**node).left;
            } else {
                current = &(**node).right;
            }
        }

        result
    }

}