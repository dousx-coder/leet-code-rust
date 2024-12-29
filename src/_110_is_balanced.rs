///
/// 110 判断平衡二叉树
///
/// https://leetcode.cn/problems/balanced-binary-tree/
///
struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(rc) => {
                let borrow = rc.borrow();
                let left = Self::node_is_balanced(borrow.left.clone());
                let left_height = left.1;
                let left_is_balanced = left.0;
                if !left_is_balanced || left_height < 0 {
                    return false;
                }
                let right = Self::node_is_balanced(borrow.right.clone());
                let right_height = right.1;
                let right_is_balanced = right.0;
                if !right_is_balanced || right_height < 0 {
                    return false;
                }
                let sub = left_height - right_height;
                sub.abs() <= 1
                    && Self::is_balanced(borrow.left.clone())
                    && Self::is_balanced(borrow.right.clone())
            }
        }
    }
    ///
    /// 判断当前node是否为平衡二叉树；
    ///
    /// 如果是平衡二叉树，则返回树的高度，反之返回[`i32::MIN`]
    ///
    fn node_is_balanced(node: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match node {
            None => (true, 0),
            Some(rc) => {
                let borrow = rc.borrow();
                let left = Self::node_is_balanced(borrow.left.clone());
                let left_is_balanced = left.0;
                if !left_is_balanced {
                    // 左子树非平衡二叉树,剪枝
                    return (false, i32::MIN);
                };
                let right = Self::node_is_balanced(borrow.right.clone());
                let right_is_balanced = right.0;
                if !right_is_balanced {
                    return (false, i32::MIN);
                };
                let left_height = left.1;
                let right_height = right.1;
                let is_balanced = (left_height - right_height).abs() <= 1;
                let tree_height = 1 + max(left_height, right_height);
                (is_balanced, tree_height)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let sequential = vec![-1, 1, 2, 3, 4, 5, 6];
        let root = TreeNode::build_complete_tree(&sequential);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn t2() {
        let sequential = vec![-1, 3, 9, 20, i32::MIN, i32::MIN, 15, 7];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn t3() {
        let sequential = vec![-1, 1, 2, 2, 3, 3, i32::MIN, i32::MIN, 4, 4];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert!(!Solution::is_balanced(root));
    }
    #[test]
    fn t4() {
        let root = None;
        assert!(Solution::is_balanced(root));
    }
}
