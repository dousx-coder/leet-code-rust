///
/// https://leetcode.cn/problems/UHnkqh/description/
///
struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

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

    ///
    /// [Vec]转链表,返回头节点
    fn convert_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        if vec.len() == 0 {
            return None;
        }
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        for x in vec {
            current.next = Some(Box::new(ListNode::new(x)));
            current = current.next.as_mut().unwrap();
        }
        dummy.next
    }


    #[test]
    fn t1() {
        let head = convert_linked_list(vec![1, 2, 3, 4, 5]);
        let mut result = Solution::reverse_list(head);
        while let Some(node) = result {
            print!("{:?} ", node.val);
            result = node.next;
        }
    }
}