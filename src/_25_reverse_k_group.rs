use crate::common::linked_list::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

///
///
/// `25. K 个一组翻转链表`
///
/// https://leetcode.cn/problems/reverse-nodes-in-k-group/description/
///
struct Solution;
impl Solution {
    /// 从后往前 反转，每次拿到后面相邻的新头节点
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut next_head = &mut head;
        for _ in 0..k {
            if let Some(node) = next_head.as_mut() {
                next_head = &mut node.next;
            } else {
                return head;
            }
        }
        let next_head = Self::reverse_k_group(next_head.take(), k);
        Self::reverse(head, next_head)
    }

    fn reverse(
        mut head: Option<Box<ListNode>>,
        mut next_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        while let Some(mut node) = head {
            head = node.next.take();
            // link head -> next_head
            node.next = next_head.take();
            // as tail
            next_head = Some(node);
        }
        next_head
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let linked_list = vec_to_linked_list(&vec![1, 2, 3, 4, 5], false);
        let result = Solution::reverse_k_group(linked_list, 3);
        let result_vec = linked_list_to_vec(&result);
        assert_eq!(vec![3, 2, 1, 4, 5], result_vec);
    }

    #[test]
    fn t2() {
        let linked_list = vec_to_linked_list(&vec![1, 2, 3, 4, 5, 6], false);
        let result = Solution::reverse_k_group(linked_list, 3);
        let result_vec = linked_list_to_vec(&result);
        assert_eq!(vec![3, 2, 1, 6, 5, 4], result_vec);
    }
}
