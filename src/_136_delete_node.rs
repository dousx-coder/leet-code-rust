///
/// https://leetcode.cn/problems/shan-chu-lian-biao-de-jie-dian-lcof/description/
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
    ///
    /// 链表转[Vec]
    fn convert_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        let mut curr = head;
        while let Some(node) = curr {
            vec.push(node.val);
            curr = &node.next;
        }
        vec
    }

    ///
    /// [Vec]转链表,返回头节点
    fn convert_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        if vec.len() == 0 {
            return None;
        }
        let mut dummy = Box::new(ListNode::new(-1));
        let mut curr = &mut dummy;
        for x in vec {
            curr.next = Some(Box::new(ListNode::new(x)));
            curr = curr.next.as_mut().unwrap();
        }
        dummy.next
    }

    #[test]
    fn t1() {
        let mut head = convert_linked_list(vec![1, 2, 3, 4, 5]);
        let mut result = Solution::delete_node(head, 2);
        let vec = convert_vec(&mut result);
        println!("{:?}", vec);
        assert_eq!(vec, vec![1, 3, 4, 5]);
    }
}