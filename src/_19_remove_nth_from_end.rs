use crate::common::list_node::ListNode;

///
/// 18 remove_nth_from_end
///
/// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
///

struct Solution {}

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
    use crate::common::util::convert_linked_list;

    #[test]
    fn t1() {
        let mut head = convert_linked_list(&vec![1, 2, 3, 4, 5], false);
        println!("before");
        let mut hr = head.as_ref();
        while let Some(node) = hr {
            print!("{} ", node.val);
            hr = node.next.as_ref();
        }
        println!("");

        let result = Solution::remove_nth_from_end(head, 2);
        // Print the result
        println!("result");
        let mut node = &result;
        while let Some(n) = node {
            print!("{} ", n.val);
            node = &n.next;
        }
    }
}
