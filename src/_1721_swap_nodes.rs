use crate::common::list_node::ListNode;
use std::cmp::{max, min};
use std::mem::replace;
///
/// `1721 交换链表中的节点`
///
/// https://leetcode.cn/problems/swapping-nodes-in-a-linked-list/
///
struct Solution;
impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut len = 0;
        let mut c = &head;
        while let Some(a) = c {
            len += 1;
            c = &a.next;
        }
        let mut left = min(k, len - k + 1);
        let mut right = max(k, len - k + 1);
        if left == right {
            return head;
        }
        let mut head_p = &mut head;
        for i in 0..left - 1 {
            head_p = &mut head_p.as_mut().unwrap().next;
        }
        let mut b = replace(head_p, None);
        let mut c;
        let mut d;
        let mut e;
        unsafe {
            let mut b_p = &mut b.as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            c = replace(&mut *b_p, None);
            let mut c_p = &mut c as *mut Option<Box<ListNode>>;
            for i in left..right - 1 {
                c_p = &mut (*c_p).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            }
            d = replace(&mut *c_p, None);
            let mut d_p = &mut d.as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            e = replace(&mut *d_p, None);
            let _ = replace(head_p, d);
            if c != None {
                let _ = replace(&mut *d_p, c);
                let _ = replace(&mut *c_p, b);
            } else {
                let _ = replace(&mut *d_p, b);
            }
            let _ = replace(&mut *b_p, e);
        }
        head
    }
}
#[cfg(test)]
mod tests {
    
    use super::*;
    use crate::common::util::{linked_list_to_vec, vec_to_linked_list};

    #[test]
    fn t1() {
        let ans = Solution::swap_nodes(vec_to_linked_list(&vec![1, 2, 3, 4, 5], false), 2);
        assert_eq!(linked_list_to_vec(&ans), vec![1, 4, 3, 2, 5])
    }
    #[test]
    fn t2() {
        let ans = Solution::swap_nodes(
            vec_to_linked_list(&vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5], false),
            5,
        );
        assert_eq!(linked_list_to_vec(&ans), vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5])
    }
}
