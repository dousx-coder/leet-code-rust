use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
/// 998 最大二叉树Ⅱ
///
/// https://leetcode.cn/problems/maximum-binary-tree-ii/
struct Solution;
impl Solution {
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        let mut rt = root.clone();
        let mut borrow = rt.as_ref().unwrap().borrow_mut();
        if borrow.val < val {
            let mut node = TreeNode::new(val);
            node.left = rt.clone();
            return Some(Rc::new(RefCell::new(node)));
        }
        let right = borrow.right.clone();
        borrow.right = Self::insert_into_max_tree(right, val);
        rt.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![4, 1, 3, 2];
        let inorder = vec![1, 4, 2, 3];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let tree = Solution::insert_into_max_tree(root, 5);
        let preorder = TreeNode::preorder_traversal_recursive(&tree);
        assert_eq!(preorder, vec![5, 4, 1, 3, 2]);
        let inorder = TreeNode::inorder_traversal_recursive(&tree);
        assert_eq!(inorder, vec![1, 4, 2, 3, 5]);
    }
}
