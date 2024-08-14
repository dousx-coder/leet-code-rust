/// https://leetcode.cn/problems/merge-k-sorted-lists/
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
struct Solution {}

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 反转比较顺序，使小值具有更高优先级
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 使用 `Ord` 中定义的 `cmp` 方法
        Some(self.cmp(other))
    }
}
impl Solution {
    ///
    /// 借助小顶堆 [`BinaryHeap`]实现链表合并
    ///
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        // 创建一个空的小顶堆
        let mut min_heap = BinaryHeap::new();
        for first in lists.into_iter() {
            let mut current = first;
            while let Some(mut node) = current {
                // 将 `node.next` 的所有权安全取出，并使其变为 `None`
                current = node.next.take();
                // current = node.next;// 这种写法编译失败: 部分所有权取出 导致无法继续使用node
                min_heap.push(node);
            }
        }
        if min_heap.is_empty() {
            return None;
        }
        // 虚拟头节点，帮助我们拼接链表
        let mut dummy = ListNode { val: 0, next: None };
        let mut tail = &mut dummy;
        while let Some(mut node) = min_heap.pop() {
            // 将取出的节点连接到新链表的尾部
            // 将 node 赋值给 tail 的 next
            tail.next = Some(node);
            // 移动 tail 指针到链表尾部
            tail = tail.next.as_mut()?;
        }
        dummy.next
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let list1 = ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(7))),
            })),
        };
        let list2 = ListNode {
            val: 16,
            next: Some(Box::new(ListNode {
                val: 12,
                next: Some(Box::new(ListNode::new(20))),
            })),
        };
        let vec = vec![
            Some(Box::new(list1)),
            Some(Box::new(list2))
        ];
        let result = Solution::merge_k_lists(vec);
        let mut actual = vec![];
        let mut expect = vec![3, 4, 7, 12, 16, 20];
        let mut current = &result;
        while let Some(node) = current {
            actual.push(node.val);
            current = &node.next;
        }
        assert_eq!(actual, expect)
    }
}
