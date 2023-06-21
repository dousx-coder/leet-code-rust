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
        let mut head_node = list_node;
        let mut next_node = &mut head_node;
        let mut carry = 0;
        while next_node.is_some() {
            let mut temp_node = next_node.as_mut().unwrap();
            let val = temp_node.val + carry;
            temp_node.val = val % 10;
            carry = val / 10;
            if carry != 0 && temp_node.next.is_none() {
                temp_node.next = Option::from(Box::new(ListNode::new(carry)));
                // 这里不应该返回list_node 因为数据已经被修改了 这里应该返回新的头结点
                return head_node;
            } else {
                next_node = &mut temp_node.as_mut().next;
            }
        }
        return head_node;
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
            let val = sum % 10;
            let mut box_node = l1.unwrap();
            if box_node.next.is_none() {
                box_node.next = Some(Box::new(ListNode::new(0)))
            }
            box_node.next.as_mut().unwrap().val += carry;
            let mut l = ListNode::new(val);
            l.next = Solution::add_two_numbers(box_node.next, l2.unwrap().next);
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

    ///打印链表节点 并将链表转换为 [`Vec`]返回
    fn print_list_and_to_vec(list_head: &mut Option<Box<ListNode>>) -> Vec<i32> {
        if list_head.is_none() {
            return vec![];
        }
        let mut vec_result: Vec<i32> = vec![];
        let mut l1r = list_head.as_ref();
        while l1r.is_some() {
            let var = l1r.unwrap().val;
            print!("{} ", var);
            vec_result.append(&mut vec![var]);

            l1r = l1r.unwrap().next.as_ref();
        };
        println!();
        vec_result
    }

    /// 构建链表
    fn build_list(mut num_list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head_node = None;
        //  数组翻转一下 用头插法构建
        // todo!(抽空研究下尾插)
        num_list.reverse();
        for num in num_list {
            let mut list_node = Some(Box::new(ListNode::new(num)));
            if head_node == None {
                head_node = list_node;
            } else {
                list_node.as_mut().unwrap().next = head_node;
                head_node = list_node;
            }
        }
        return head_node;
    }


    #[test]
    fn test1() {
        let mut l1_0 = build_list(vec![2, 4, 3]);
        print_list_and_to_vec(&mut l1_0);

        let mut l2_0 = build_list(vec![5, 6, 4]);
        print_list_and_to_vec(&mut l2_0);

        let mut _result = Solution::add_two_numbers(l1_0, l2_0);
        let r = print_list_and_to_vec(&mut _result);
        assert_eq!(vec![7, 0, 8], r)
    }

    #[test]
    fn test2() {
        let mut l1_0 = build_list(vec![9, 9, 9, 9, 9, 9, 9]);
        print_list_and_to_vec(&mut l1_0);

        let mut l2_0 = build_list(vec![9, 9, 9, 9]);
        print_list_and_to_vec(&mut l2_0);

        let mut _result = Solution::add_two_numbers(l1_0, l2_0);
        let r = print_list_and_to_vec(&mut _result);
        assert_eq!(vec![8, 9, 9, 9, 0, 0, 0, 1], r)
    }

    #[test]
    fn test3() {
        let mut l1_0 = build_list(vec![9, 1]);
        print_list_and_to_vec(&mut l1_0);

        let mut l2_0 = build_list(vec![1, 2, 3]);
        print_list_and_to_vec(&mut l2_0);

        let mut _result = Solution::add_two_numbers(l1_0, l2_0);
        let r = print_list_and_to_vec(&mut _result);
        assert_eq!(vec![0, 4, 3], r)
    }
}
