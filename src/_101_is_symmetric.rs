use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// `101 对称二叉树`
///
/// https://leetcode.cn/problems/symmetric-tree/
///
struct Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(rc) => {
                let borrow = rc.borrow();
                Self::symmetric_tree(borrow.left.clone(), borrow.right.clone())
            }
        }
    }
    fn symmetric_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            true
        } else if p.is_some() && q.is_some() {
            let pc_un = p.clone().unwrap();
            let qc_un = q.clone().unwrap();
            let p_borrow = pc_un.borrow();
            let q_borrow = qc_un.borrow();
            let p_val = p_borrow.val;
            let q_val = q_borrow.val;
            p_val == q_val
                && Self::symmetric_tree(p_borrow.left.clone(), q_borrow.right.clone())
                && Self::symmetric_tree(p_borrow.right.clone(), q_borrow.left.clone())
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    ///
    /// ```xml
    ///       1
    ///     /   \
    ///    2     2
    ///   / \   / \
    ///  3  4  4   3
    /// ```
    #[test]
    fn t1() {
        let sequential = vec![i32::MIN, 1, 2, 2, 3, 4, 4, 3];
        let root = TreeNode::build_complete_tree(&sequential);
        let vec1 = TreeNode::preorder_traversal_recursive(&root);
        assert_eq!(
            TreeNode::preorder_traversal_recursive(&root),
            vec![1, 2, 3, 4, 2, 4, 3]
        );
        assert_eq!(
            TreeNode::inorder_traversal_recursive(&root),
            vec![3, 2, 4, 1, 4, 2, 3]
        );
        assert!(Solution::is_symmetric(root.clone()))
    }

    #[test]
    fn t2() {
        let sequential = vec![i32::MIN, 1, 2, 3, 4, 5, 6];
        let root = TreeNode::build_complete_tree(&sequential);
        assert_eq!(Solution::is_symmetric(root.clone()), false);
    }
}
