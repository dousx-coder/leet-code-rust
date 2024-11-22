use crate::common::list_node::ListNode;
use crate::common::util::convert_linked_list;
use crate::common::util::convert_vec;

///
/// https://leetcode.cn/problems/SLwz0R/
///
struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n <= 0 {
            return head;
        }
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut current = &mut dummy;
        let mut len = 0;
        while current.as_ref().unwrap().next.is_some() {
            len += 1;
            current = &mut current.as_mut().unwrap().next;
        }
        let target_index = len - n;
        if target_index < 0 {
            return dummy?.next;
        };
        let mut del_pre_node = &mut dummy;
        for i in 0..target_index {
            del_pre_node = &mut del_pre_node.as_mut().unwrap().next;
        }
        if let Some(pre_node) = del_pre_node {
            let mut need_del_node = &mut pre_node.next;
            pre_node.next = need_del_node.as_mut()?.next.take();
        }
        dummy?.next
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let head = convert_linked_list(&vec![1, 2, 3, 4, 5], false);
        let mut ans = Solution::remove_nth_from_end(head, 2);
        let vec = convert_vec(&mut ans);
        println!("{:?}", vec);
        assert_eq!(vec, vec![1, 2, 3, 5]);
    }
    #[test]
    fn t2() {
        let head = convert_linked_list(&vec![1, 2, 3, 4, 5], false);
        let mut ans = Solution::remove_nth_from_end(head, 0);
        let vec = convert_vec(&mut ans);
        println!("{:?}", vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn t3() {
        let head = convert_linked_list(&vec![1, 2, 3, 4, 5], false);
        let mut ans = Solution::remove_nth_from_end(head, 1);
        let vec = convert_vec(&mut ans);
        println!("{:?}", vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);
    }

    #[test]
    fn t4() {
        let head = convert_linked_list(&vec![1, 2, 3, 4, 5], false);
        let mut ans = Solution::remove_nth_from_end(head, 5);
        let vec = convert_vec(&mut ans);
        println!("{:?}", vec);
        assert_eq!(vec, vec![2, 3, 4, 5]);
    }

    #[test]
    fn t5() {
        let head = convert_linked_list(&vec![1, 2, 3, 4, 5], false);
        let mut ans = Solution::remove_nth_from_end(head, 6);
        let vec = convert_vec(&mut ans);
        println!("{:?}", vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }
}
