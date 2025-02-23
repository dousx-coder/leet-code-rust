use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
///
/// `98 验证二叉搜索树`
///
/// https://leetcode.cn/problems/validate-binary-search-tree/
///
///
struct Solution;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_bst(root, i64::MIN, i64::MAX)
    }
    fn is_bst(node: Option<Rc<RefCell<TreeNode>>>, mix: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(rc_node) => {
                let borrow = rc_node.borrow();
                let node_val = borrow.val as i64;

                if node_val <= mix || node_val >= max {
                    return false;
                }
                Self::is_bst(borrow.left.clone(), mix, node_val)
                    && Self::is_bst(borrow.right.clone(), node_val, max)
            }
        }
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
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn t2() {
        let preorder = vec![2, 1, 3];
        let inorder = vec![1, 2, 3];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::is_valid_bst(root), true);
    }

    #[test]
    fn t3() {
        let preorder = vec![5, 1, 4, 3, 6];
        let inorder = vec![1, 5, 3, 4, 6];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn t4() {
        let sequential = vec![-1, 1, 1];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn t5() {
        let preorder = vec![5, 4, 6, 3, 7];
        let inorder = vec![4, 5, 3, 6, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn t6() {
        let preorder = vec![2147483647];
        let inorder = vec![2147483647];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::is_valid_bst(root), true);
    }
}
