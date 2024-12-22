use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
///
///
/// https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/description/
///
struct Solution;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        // 下一层
        let mut next_level = VecDeque::new();
        // 当前层
        let mut curr_level = VecDeque::new();
        curr_level.push_back(root.clone());
        let mut curr_level_val = vec![];

        loop {
            match curr_level.pop_front() {
                Some(node) => match node {
                    None => {}
                    Some(rc) => {
                        curr_level_val.push(rc.borrow().val);
                        next_level.push_back(rc.borrow().left.clone());
                        next_level.push_back(rc.borrow().right.clone());
                    }
                },
                None => {}
            }

            if curr_level.is_empty() && !next_level.is_empty() {
                result.push(curr_level_val.clone());
                curr_level_val.clear();
                while let Some(rc) = next_level.pop_front() {
                    curr_level.push_back(rc);
                }
            }
            if curr_level.is_empty() && next_level.is_empty() {
                break;
            }
        }
        result.reverse();
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::level_order_bottom(root);
        let mut expect = vec![vec![3], vec![9, 20], vec![6, 15, 7], vec![1]];
        expect.reverse();
        assert_eq!(ans, expect);
    }
}
