use crate::common::linked_list::ListNode;

///
/// 18 remove_nth_from_end
///
/// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
///

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        unsafe {
            let mut slow = &mut dummy as *mut Box<ListNode>;
            let mut fast = &mut dummy as *mut Box<ListNode>;
            // move fast n forward
            for _ in 0..n {
                fast = (*fast).next.as_mut()?;
            }

            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut()?;
                slow = (*slow).next.as_mut()?;
            }
            (*slow).next = (*slow).next.take()?.next;
        }

        dummy.next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::linked_list::{linked_list_to_vec, vec_to_linked_list};

    #[test]
    fn t1() {
        let head = vec_to_linked_list(&vec![1, 2, 3, 4, 5], false);
        let before = linked_list_to_vec(&head);
        let after = Solution::remove_nth_from_end(head, 2);
        let after = linked_list_to_vec(&after);
        assert_eq!(after, vec![1, 2, 3, 5]);
    }

    #[test]
    fn t2() {
        let head = vec_to_linked_list(&vec![1, 2, 3, 4, 5], false);
        let before = linked_list_to_vec(&head);
        let after = Solution::remove_nth_from_end(head, 1);
        let after = linked_list_to_vec(&after);
        assert_eq!(after, vec![1, 2, 3, 4]);
    }
}
