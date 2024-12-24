use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
///
/// `111 二叉树的最小深度`
///
/// https://leetcode.cn/problems/minimum-depth-of-binary-tree/
struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if left.is_none() && right.is_none() {
                    return 1;
                }
                if left.is_some() && right.is_some() {
                    let left_min = Self::min_depth(left);
                    let right_min = Self::min_depth(right);
                    return min(left_min, right_min) + 1;
                }
                if left.is_none() {
                    Self::min_depth(right) + 1
                } else {
                    Self::min_depth(left) + 1
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::min_depth(root), 2);
    }

    #[test]
    fn t2() {
        let preorder = vec![2, 3, 4, 5, 6];
        let inorder = vec![2, 3, 4, 5, 6];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::min_depth(root), 5);
    }
}
