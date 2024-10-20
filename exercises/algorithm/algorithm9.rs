/*
	heap
	This question requires you to implement a binary heap function
*/
 

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引 0 不使用
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn add(&mut self, value: T) {
        // 增加元素到堆中
        if self.count + 1 >= self.items.len() {
            self.items.push(T::default());
        }
        self.count += 1;
        self.items[self.count] = value;
        self.bubble_up(self.count);
    }

    fn bubble_up(&mut self, idx: usize) {
        let mut idx = idx;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> Option<usize> {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if left > self.count {
            return None;
        }

        if right > self.count {
            return Some(left);
        }

        if (self.comparator)(&self.items[left], &self.items[right]) {
            Some(left)
        } else {
            Some(right)
        }
    }

    fn bubble_down(&mut self, idx: usize) {
        let mut idx = idx;
        while let Some(smallest_child) = self.smallest_child_idx(idx) {
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                self.items.swap(idx, smallest_child);
                idx = smallest_child;
            } else {
                break;
            }
        }
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 取出顶部元素
        let top = self.items[1].clone();

        // 用最后一个元素替换顶部元素
        self.items[1] = self.items[self.count].clone(); // 将最后一个元素移到顶部
        self.count -= 1; // 减少计数
        self.bubble_down(1); // 向下调整以维持堆的性质

        Some(top) // 返回顶部元素
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
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
        assert_eq!(heap.next(), Some(11)); // 这个测试需要更改
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
        assert_eq!(heap.next(), Some(2)); // 这个测试需要更改
    }
}