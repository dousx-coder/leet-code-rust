use crate::common::list_node::ListNode;
use crate::common::util::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

///
///
/// `25. K 个一组翻转链表`
///
/// https://leetcode.cn/problems/reverse-nodes-in-k-group/description/
///
struct Solution;
impl Solution {
    /// 从前往后反转
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 1 || head.is_none() {
            return head;
        }
        // 节点数足够 k 个，则反转这 k 个节点
        let mut call_reverse_k_group = false;
        // 统计链表中前 k 个节点，看看是否足够 k 个
        let mut count = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            count += 1;
            if count == k {
                call_reverse_k_group = true;
                break;
            }
            curr = &node.next;
        }
        if call_reverse_k_group {
            let mut prev: Option<Box<ListNode>> = None;
            let mut curr = head.take();
            let mut next: Option<Box<ListNode>>;
            // tail 表示当前段反转后的尾部
            let mut tail = None;
            // 反转 k 个节点
            for i in 0..k {
                if let Some(mut current_node) = curr {
                    // 断开当前节点与下一个节点的连接
                    next = current_node.next.take();
                    // 反转当前节点的指针方向
                    current_node.next = prev;
                    // 更新 prev 为当前节点
                    prev = Some(current_node);
                    // 继续处理下一个节点
                    curr = next;
                    if i == 0 {
                        // 将第一个节点作为反转链表的尾部
                        // 从前往后反转，没办法保存上次的tail，只能clone一份，最后拼接，这样链表被改了
                        tail = prev.clone();
                    }
                }
            }
            // 递归反转其余的部分，并将当前段的尾部与递归的结果连接
            let next_part = Self::reverse_k_group(curr, k);
            if next_part.is_some() && tail.is_some() {
                let mut curr = &mut prev;
                let las_prev = k - 2;
                // 找到倒数第二个节点，将tail拼接上
                for i in 0..las_prev {
                    if let Some(ref mut node) = curr {
                        curr = &mut node.next;
                    }
                }
                if let Some(ref mut curr_node) = curr {
                    tail.as_mut()?.next = next_part;
                    curr_node.next = tail;
                }
            }
            // 返回反转后的头部
            prev
        } else {
            //如果剩余的节点不足 k 个，则不反转，直接返回
            head
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let linked_list = vec_to_linked_list(&vec![1, 2, 3, 4, 5], false);
        let result = Solution::reverse_k_group(linked_list, 3);
        let result_vec = linked_list_to_vec(&result);
        assert_eq!(vec![3, 2, 1, 4, 5], result_vec);
    }

    #[test]
    fn t2() {
        let linked_list = vec_to_linked_list(&vec![1, 2, 3, 4, 5, 6], false);
        let result = Solution::reverse_k_group(linked_list, 3);
        let result_vec = linked_list_to_vec(&result);
        assert_eq!(vec![3, 2, 1, 6, 5, 4], result_vec);
    }
}
