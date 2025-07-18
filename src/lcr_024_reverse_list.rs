use crate::common::linked_list::*;
///
/// [LCR 024. 反转链表](https://leetcode.cn/problems/UHnkqh/description/)
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
        let result_vec = linked_list_to_vec(&result);
        assert_eq!(vec![5, 4, 3, 2, 1], result_vec);
    }
}
