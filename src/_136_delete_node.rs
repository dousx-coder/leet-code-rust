use crate::common::list_node::ListNode;
use crate::common::util::{linked_list_to_vec, vec_to_linked_list};
///
/// https://leetcode.cn/problems/shan-chu-lian-biao-de-jie-dian-lcof/description/
struct Solution;

impl Solution {
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val, next: head }));
        let mut curr = &mut dummy;
        while let Some(ref mut node) = curr.as_mut()?.next {
            if node.val == val {
                // take() 会将 node.next 的值移出
                curr.as_mut()?.next = node.next.take();
            }
            curr = &mut curr.as_mut()?.next;
        }
        dummy?.next
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut dummy = vec_to_linked_list(&vec![1, 2, 3, 4, 5], true);
        let head = dummy.unwrap().next;
        let mut result = Solution::delete_node(head, 2);
        let vec = linked_list_to_vec(&mut result);
        println!("{:?}", vec);
        assert_eq!(vec, vec![1, 3, 4, 5]);
    }
}
