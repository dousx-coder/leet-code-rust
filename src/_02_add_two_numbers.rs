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
            nun.next = Option::from(n.clone());
            return list_node;
        } else {
            next = nun.next;
        }
    }
    return list_node;
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // if l1.is_none() {
    //     return special_dispose(l2);
    // }
    // if l2.is_none() {
    //     return special_dispose(l1);
    // }
    // let v1 = l1.clone().unwrap().val;
    // let v2 = l2.clone().unwrap().val;
    // let sum = v1 + v2;
    // if sum > 9 {
    //     let carry = sum / 10;
    //     let va = sum % 10;
    //     if l1.unwrap().next.is_none() {
    //         l1.unwrap().next = Some(Box::new(ListNode::new(0)))
    //     }
    //     l1.unwrap().next.unwrap().val += carry;
    //     let mut l = ListNode::new(va);
    //     l.next = add_two_numbers(l1.unwrap().next, l2.unwrap().next);
    //     return Some(Box::new(l));
    // };
    //
    // let mut l = ListNode::new(sum);
    // l.next = add_two_numbers(l1.unwrap().next, l2.unwrap().next);
    // return Some(Box::new(l));
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut l1_0 = Some(Box::new(ListNode::new(2)));
        let mut l1_1 = Some(Box::new(ListNode::new(4)));
        let mut l1_2 = Some(Box::new(ListNode::new(4)));
        l1_0.clone().unwrap().next = l1_1.clone();
        l1_1.clone().unwrap().next = l1_2.clone();


        let mut l2_0 = Some(Box::new(ListNode::new(5)));
        let mut l2_1 = Some(Box::new(ListNode::new(6)));
        let mut l2_2 = Some(Box::new(ListNode::new(7)));
        l2_0.clone().unwrap().next = l2_1.clone();
        l2_1.clone().unwrap().next = l2_2.clone();


        let _result = add_two_numbers(l1_0, l2_0);
        println!()
    }
}
