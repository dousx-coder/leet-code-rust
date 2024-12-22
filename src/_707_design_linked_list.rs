use crate::common::linked_list::ListNode;
use std::ops::Index;

#[derive(PartialEq, Eq, Clone, Debug)]
struct MyLinkedList {
    dummy: Option<Box<ListNode>>,
    len: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            dummy: Some(Box::new(ListNode::new(-1))),
            len: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.len {
            return -1;
        }
        let mut curr = &self.dummy;
        for i in 0..=index {
            curr = &curr.as_ref().unwrap().next;
        }
        curr.as_ref().unwrap().val
    }

    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len, val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.len {
            return;
        }
        let mut current = &mut self.dummy;
        for _ in 0..index {
            let x = current.as_mut().unwrap();
            current = &mut (x.next);
        }
        let x = current.as_mut().unwrap();
        let new_node = Some(Box::new(ListNode {
            val,
            next: x.next.take(),
        }));
        x.next = new_node;
        self.len += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.len {
            return;
        }
        let mut curr = &mut self.dummy;

        for _ in 0..index {
            curr = &mut curr.as_mut().unwrap().next;
        }
        if let Some(node) = curr {
            curr.as_mut().unwrap().next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
        }
        self.len -= 1;
    }
    fn to_vec(&self) -> Vec<i32> {
        if self.len <= 0 {
            return vec![];
        }
        let mut result = vec![];
        let mut current = if let Some(node) = self.dummy.as_ref() {
            &node.next
        } else {
            &None
        };

        while current.is_some() {
            let option = current.as_ref();
            if let Some(x) = option {
                result.push(x.val);
                current = &x.next
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::linked_list::linked_list_to_vec;
    #[test]
    fn t1() {
        let mut list = MyLinkedList::new();
        let value: i32 = list.get(0);
        let value: i32 = list.get(1);
        list.add_at_head(1);
        assert_eq!(list.to_vec(), vec![1]);

        list.add_at_tail(3);
        assert_eq!(list.to_vec(), vec![1, 3]);

        let ret_1: i32 = list.get(2);
        let ret_1: i32 = list.get(3);
        list.add_at_index(1, 2);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);

        list.delete_at_index(1);
        assert_eq!(list.to_vec(), vec![1, 3]);
    }
}
