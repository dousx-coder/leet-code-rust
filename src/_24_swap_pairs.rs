struct Solution;

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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut curr = &mut dummy;

        while let Some(ref mut node) = curr {
            // 检查是否有两个节点可以交换
            if let Some(mut first) = node.next.take() {
                if let Some(mut second) = first.next.take() {
                    // 交换 first 和 second
                    first.next = second.next.take();
                    second.next = Some(first);
                    node.next = Some(second);
                    // 更新 curr 到下一对
                    curr = &mut node.next.as_mut().unwrap().next;
                } else {
                    // 存在第一个节点，但是没有第二个节点，要把之前take取出的返回去
                    node.next = Some(first);
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
    use super::*;


    #[test]
    fn t1() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));

        let result = Solution::swap_pairs(head);
        println!("{:#?}", result); // 输出交换后的链表
    }
}
