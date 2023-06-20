#[warn(unused_imports)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
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
// #[allow(dead_code)]
impl Solution {
    pub fn special_dispose(list_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list_node.is_none() {
            return list_node;
        }
        let mut next = list_node.clone().unwrap().next;
        let mut carry = 0;
        while next.is_some() {
            let mut nun = next.clone().unwrap();
            let val = nun.val + carry;
            nun.val = val % 10;
            carry = val / 10;
            if carry != 0 && nun.next.is_none() {
                let n = Box::new(ListNode::new(carry));
                // todo! 待处理
                // 9 9 9 9 9 9 9
                // 9 9 9 9
                nun.next = Option::from(n.clone());
                return list_node;
            } else {
                next = nun.next;
            }
        }
        return list_node;
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return Solution::special_dispose(l2);
        }
        if l2.is_none() {
            return Solution::special_dispose(l1);
        }

        let v1 = l1.as_ref().unwrap().val;
        let v2 = l2.as_ref().unwrap().val;
        let sum = v1 + v2;
        if sum > 9 {
            let carry = sum / 10;
            let va = sum % 10;
            let mut l1u = l1.clone().unwrap();
            if l1u.next.is_none() {
                l1u.next = Some(Box::new(ListNode::new(0)))
            }
            l1u.next.as_mut().unwrap().val += carry;
            let mut l = ListNode::new(va);
            l.next = Solution::add_two_numbers(l1u.next, l2.unwrap().next);
            return Some(Box::new(l));
        };

        let mut l = ListNode::new(sum);
        l.next = Solution::add_two_numbers(l1.unwrap().next, l2.unwrap().next);
        return Some(Box::new(l));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut l1_0 = Some(Box::new(ListNode::new(2)));
        let l1_1 = Some(Box::new(ListNode::new(4)));
        let l1_2 = Some(Box::new(ListNode::new(3)));
        let l1u = l1_0.as_mut().unwrap();
        l1u.next = l1_1;
        l1u.next.as_mut().unwrap().next = l1_2;
        println!("{:?}", l1u);
        let mut l2_0 = Some(Box::new(ListNode::new(5)));
        let l2_1 = Some(Box::new(ListNode::new(6)));
        let l2_2 = Some(Box::new(ListNode::new(4)));

        let l2u = l2_0.as_mut().unwrap();
        l2u.next = l2_1;
        l2u.next.as_mut().unwrap().next = l2_2;
        println!("{:?}", l2u);
        let _result = Solution::add_two_numbers(Some(l1u.clone()), Some(l2u.clone()));
        println!("{:?}", _result);
    }
}
