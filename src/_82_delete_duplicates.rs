use crate::common::linked_list::ListNode;
///
/// [82 删除排序链表中的重复元素](https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/)
///
struct Solution;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::next(head)
    }

    fn next(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => {
                let mut current = Some(node);
                let mut count = 1;

                // 统计相同值的节点数量
                while let Some(next_node) = current.as_mut()?.next.take() {
                    if next_node.val == current.as_ref()?.val {
                        count += 1;
                        current.as_mut()?.next = next_node.next;
                    } else {
                        current.as_mut()?.next = Some(next_node);
                        break;
                    }
                }

                if count == 1 {
                    // 当前节点值唯一，保留并递归处理后续节点
                    let next = current.as_mut()?.next.take();
                    current.as_mut()?.next = Self::next(next);
                    current
                } else {
                    // 当前节点值重复，删除并递归处理后续节点
                    let next = current.as_mut()?.next.take();
                    Self::next(next)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::linked_list::{linked_list_to_vec, vec_to_linked_list};

    #[test]
    fn t1() {
        let vec = vec![1, 2, 3, 3, 4, 4, 5];
        let ans = Solution::delete_duplicates(vec_to_linked_list(&vec, false));
        let ans = linked_list_to_vec(&ans);
        assert_eq!(ans, vec![1, 2, 5]);
    }
    
    #[test]
    fn t2() {
        let vec = vec![1, 1, 1, 2, 3];
        let ans = Solution::delete_duplicates(vec_to_linked_list(&vec, false));
        let ans = linked_list_to_vec(&ans);
        assert_eq!(ans, vec![2, 3]);
    }
}
