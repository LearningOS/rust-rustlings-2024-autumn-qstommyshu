/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // first insert the new value to the end of heap, then sift up
        // to maintain the heap properties
        // 1 based indexing
        self.items.push(value);
        self.count += 1;
        self.sift_up(self.count);
    }

    // if reverse, then it is sift down
    fn sift_up(&mut self, mut idx: usize) {
        // if the inserted element is not at the top, it can be sift up
        // check if comparator is satisfied, if it is, then the value should be sift up
        // i.e. max_heap => (a > b) => if idx is larger, it should be at a higher level

        // make sure to update parent after idx is updated
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if (self.comparator) (&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }

    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // at each level, smallest child could be either the left child or right child, so we
        // need to check
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right <= self.count {
            if (self.comparator) (&self.items[left], &self.items[right]) { left }
            else { right }
        } else if self.children_present(idx) {
            left
        } else {
            idx
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        self.items.swap(1, self.count);
        self.count -= 1;
        let item = self.items.pop().unwrap();
        if self.count > 0 { self.sift_down(1); }
        Some(item)
    }

    fn sift_down(&mut self, mut idx: usize) {
        // check if reversed comparator is satisfied, if it is, then the value should be sift down
        // i.e. max_heap => (a <= b) => if idx is smaller, it should be at a lower level
        while self.children_present(idx) {
            let smallest_child = self.smallest_child_idx(idx);
            if !(self.comparator) (&self.items[idx], &self.items[smallest_child]) {
                self.items.swap(idx, smallest_child);
                idx = smallest_child;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}