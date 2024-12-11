use crate::common::list_node::ListNode;
use crate::common::util::*;
///
/// https://leetcode.cn/problems/UHnkqh/description/
///
struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 声明head为可变
        let mut head = head;
        // 取出链表头节点，head 变为 None
        let mut curr = head.take();
        // 用于保存反转后的前一个节点
        let mut pre = None;
        while let Some(mut node) = curr {
            curr = node.next.take();
            // 当前节点的 next 指向反转后的链表头
            node.next = pre;
            // 更新 prev 为当前节点
            pre = Some(node);
        }
        pre
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let head = vec_to_linked_list(&vec![1, 2, 3, 4, 5], false);
        let mut result = Solution::reverse_list(head);
        while let Some(node) = result {
            print!("{:?} ", node.val);
            result = node.next;
        }
    }
}
