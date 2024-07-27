// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        let mut cur = dummy.as_ref();
        while let Some(_) = cur {
            len += 1;
            cur = cur.unwrap().next.as_ref();
        }
        let len = len - n - 1;
        let mut cur = dummy.as_mut();
        for _ in 0..len {
            cur = cur.unwrap().next.as_mut();
        }
        let next = cur.as_mut().unwrap().next.as_mut();
        cur.unwrap().next = next.unwrap().next.take();
        dummy.unwrap().next
    }
}
#[allow(dead_code)]
fn build_list(list: &Vec<i32>) -> Option<Box<ListNode>> {
    let len = list.len();
    if len <= 0 {
        None
    } else {
        let first_value = list[0];
        let mut head = Some(Box::new(ListNode::new(first_value)));
        let mut cur = &mut head;
        for i in 1..len {
            if let Some(ref mut n) = cur {
                let var = list[i];
                n.next = Some(Box::new(ListNode::new(var)));
                cur = &mut n.next;
                // let var: Option<&i32> = list.get(i);
                // match var {
                //     Some(&v) => {
                //         n.next = Some(Box::new(ListNode::new(v)));
                //         cur = &mut n.next;
                //     }
                //     None => {}
                // }
            }
        }
        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let head = build_list(&vec![1, 2, 3, 4, 5]);
        println!("before");
        let mut hr = head.as_ref();
        while let Some(node) = hr {
            print!("{} ", node.val);
            hr = node.next.as_ref();
        }
        println!("");

        let result = Solution::remove_nth_from_end(head, 2);
        // Print the result
        println!("result");
        let mut node = &result;
        while let Some(n) = node {
            print!("{} ", n.val);
            node = &n.next;
        }
    }
}
