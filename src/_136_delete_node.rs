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
                break;
            }
            curr = &mut curr.as_mut()?.next;
        }
        dummy?.next
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: None,
                    })),
                })),
            })),
        }));
        let option = Solution::delete_node(head, 2);
        println!("{:#?}", option);
    }
}