use crate::common::list_node::ListNode;

struct Solution;


impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut curr = &mut dummy;

        while let Some(ref mut node) = curr {
            // 检查是否有两个节点可以交换 take_if检查是否有second存在
            if let Some(mut first) = node.next.take_if(|v| { v.next.is_some() }) {
                if let Some(mut second) = first.next.take() {
                    // 交换 first 和 second
                    first.next = second.next.take();
                    second.next = Some(first);
                    node.next = Some(second);
                    // 更新 curr 到下一对
                    curr = &mut node.next.as_mut().unwrap().next;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        dummy?.next
    }
}

#[cfg(test)]
mod tests {
    use crate::common::util::convert_linked_list;
    use super::*;


    #[test]
    fn t1() {
        let dummy = convert_linked_list(&vec![1, 2, 3, 4, 5], true);
        let mut head = dummy.unwrap().next;
        let mut result = Solution::swap_pairs(head);
        while let Some(mut node) = result {
            print!("{:?}  ", node.val);
            result = node.next;
        }
    }
}
