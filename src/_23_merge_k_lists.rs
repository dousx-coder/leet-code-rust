/// https://leetcode.cn/problems/merge-k-sorted-lists/

struct Solution {}

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::common::list_node::ListNode;

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
                // 将 `node.next` 的所有权安全取出，并使其变为 `None`,等价下面两行
                // current = node.next;
                // node.next = None;
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
///
/// [`Vec<i32>`]转[`Option<Box<ListNode>>`]链表
///
fn gen_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: None };
    let mut tail = &mut dummy;
    for x in vec {
        let node = ListNode { val: x, next: None };
        tail.next = Some(Box::new(node));
        tail = tail.next.as_mut()?
    }
    dummy.next
}
///
///[`Option<Box<ListNode>>`]链表转 [`Vec<i32>`]方便结果比对
///
fn list_node_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut x = &head;
    let mut result = vec![];
    while let Some(node) = x {
        result.push(node.val);
        x = &node.next;
    }
    result
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
        let vec = vec![Some(Box::new(list1)), Some(Box::new(list2))];
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

    #[test]
    fn t2() {
        let vec = vec![gen_list(vec![1, 5, 10]), gen_list(vec![2, 3, 4, 7, 8])];
        let result = Solution::merge_k_lists(vec);
        let mut expect = vec![1, 2, 3, 4, 5, 7, 8, 10];
        let actual = list_node_to_vec(result);
        assert_eq!(actual, expect)
    }
}
