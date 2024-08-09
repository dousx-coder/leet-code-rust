///
/// 18 remove_nth_from_end
///
/// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
///
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        unsafe {
            let mut slow = &mut dummy as *mut Box<ListNode>;
            let mut fast = &mut dummy as *mut Box<ListNode>;
            // move fast n forward
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }

            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
        }

        dummy.next
    }
}

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
