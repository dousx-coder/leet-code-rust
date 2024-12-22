use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// `100 相同的树`
///
/// https://leetcode.cn/problems/same-tree/
struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
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
                && Self::is_same_tree(p_borrow.left.clone(), q_borrow.left.clone())
                && Self::is_same_tree(p_borrow.right.clone(), q_borrow.right.clone())
        } else {
            false
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
        assert_eq!(Solution::is_same_tree(root.clone(), root.clone()), true);
    }

    #[test]
    fn t2() {
        let preorder1 = vec![3, 9, 20, 15, 7];
        let inorder1 = vec![9, 3, 15, 20, 7];
        let tree1 = TreeNode::build_binary_tree(&preorder1, &inorder1);

        let preorder2 = vec![1, 2, 3];
        let inorder2 = vec![2, 1, 3];
        let tree2 = TreeNode::build_binary_tree(&preorder2, &inorder2);

        assert_eq!(Solution::is_same_tree(tree1, tree2), false);
    }
}
