use crate::common::list_node::ListNode;
use crate::common::util::*;

struct Solution {}

impl Solution {
    pub fn special_dispose(list_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list_node.is_none() {
            return list_node;
        }
        let mut head_node = list_node;
        let mut next_node = &mut head_node;
        let mut carry = 0;
        while next_node.is_some() {
            let temp_node = next_node.as_mut()?;
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
        head_node
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return Solution::special_dispose(l2);
        }
        if l2.is_none() {
            return Solution::special_dispose(l1);
        }
        let mut l1_unwrap = l1?;
        let l2_unwrap = l2?;
        let v1 = l1_unwrap.val;
        let v2 = l2_unwrap.val;
        let sum = v1 + v2;
        if sum < 10 {
            //  提前判断下 如果不提前判断 leetcode上 内存占用会高一点
            let mut new_node = ListNode::new(sum);
            new_node.next = Solution::add_two_numbers(l1_unwrap.next, l2_unwrap.next);
            return Some(Box::new(new_node));
        };
        let carry = sum / 10;
        let val = sum % 10;
        if carry != 0 {
            if l1_unwrap.next.is_none() {
                l1_unwrap.next = Some(Box::new(ListNode::new(0)))
            }
            l1_unwrap.next.as_mut()?.val += carry;
        };
        let mut new_node = ListNode::new(val);
        new_node.next = Solution::add_two_numbers(l1_unwrap.next, l2_unwrap.next);
        Some(Box::new(new_node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试
    /// <br/>
    /// [`v1`] 链表1
    /// <br/>
    /// [`v2`] 链表2
    /// <br/>
    /// [`v3`] 预期结果
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let v1 = vec![2, 4, 3];
    /// let v2 = vec![5, 6, 4];
    /// let v3 = vec![7, 0, 8];
    /// _add_test(v1, v2, v3);
    /// `````
    fn _add_test(v1: Vec<i32>, v2: Vec<i32>, v3: Vec<i32>) {
        let mut l1 = vec_to_linked_list(&v1, false);

        let mut l2 = vec_to_linked_list(&v2, false);

        let mut _result = Solution::add_two_numbers(l1, l2);
        let vec = linked_list_to_vec(&_result);

        assert_eq!(v3, vec)
    }

    #[test]
    fn test1() {
        let v1 = vec![2, 4, 3];
        let v2 = vec![5, 6, 4];
        let v3 = vec![7, 0, 8];
        _add_test(v1, v2, v3);
    }

    #[test]
    fn test2() {
        let v1 = vec![9, 9, 9, 9, 9, 9, 9];
        let v2 = vec![9, 9, 9, 9];
        let v3 = vec![8, 9, 9, 9, 0, 0, 0, 1];
        _add_test(v1, v2, v3);
    }

    #[test]
    fn test3() {
        let v1 = vec![9, 1];
        let v2 = vec![1, 2, 3];
        let v3 = vec![0, 4, 3];
        _add_test(v1, v2, v3);
    }
}
