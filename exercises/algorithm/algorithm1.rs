/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)] // 添加 PartialEq 派生
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}



impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T>
where
    T: Ord + Clone, // 添加 trait 约束
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
    
        if let Some(ref mut current) = self.start {
            let mut last = current;
            while let Some(ref mut next) = last.next {
                last = next;
            }
            last.next = Some(node);
        } else {
            self.start = Some(node);
        }
        self.length += 1;
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        let mut current = &self.start;
        let mut idx = 0;

        while let Some(ref node) = current {
            if idx == index {
                return Some(&node.val);
            }
            current = &node.next;
            idx += 1;
        }
        None
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        while ptr_a.is_some() || ptr_b.is_some() {
            let val_a = ptr_a.as_ref().map(|node| &node.val);
            let val_b = ptr_b.as_ref().map(|node| &node.val);

            match (val_a, val_b) {
                (Some(a), Some(b)) => {
                    if a < b {
                        merged_list.add(a.clone());
                        ptr_a = ptr_a.and_then(|node| node.next);
                    } else {
                        merged_list.add(b.clone());
                        ptr_b = ptr_b.and_then(|node| node.next);
                    }
                }
                (Some(a), None) => {
                    merged_list.add(a.clone());
                    ptr_a = ptr_a.and_then(|node| node.next);
                }
                (None, Some(b)) => {
                    merged_list.add(b.clone());
                    ptr_b = ptr_b.and_then(|node| node.next);
                }
                (None, None) => break,
            }
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = &self.start;
        write!(f, "[")?;
        while let Some(node) = current {
            write!(f, "{}", node.val)?;
            current = &node.next;
            if current.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &target) in target_vec.iter().enumerate() {
            assert_eq!(target, *list_c.get(i as u32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &target) in target_vec.iter().enumerate() {
            assert_eq!(target, *list_c.get(i as u32).unwrap());
        }
    }

    #[test]
    fn test_merge_empty_lists() {
        let list_a: LinkedList<i32> = LinkedList::new();
        let list_b: LinkedList<i32> = LinkedList::new();
        let merged_list = LinkedList::<i32>::merge(list_a, list_b);
        assert_eq!(merged_list.length, 0);
        assert_eq!(merged_list.start, None);
    }
}