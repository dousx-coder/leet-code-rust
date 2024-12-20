use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

///
/// `94 二叉树的中序遍历`
///
/// https://leetcode.cn/problems/binary-tree-inorder-traversal/
///
struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut inorder = vec![];
                let left = Self::inorder_traversal(root.borrow().left.clone());
                if !left.is_empty() {
                    for x in left {
                        inorder.push(x);
                    }
                }
                inorder.push(root.borrow().val);
                let right = Self::inorder_traversal(root.borrow().right.clone());
                if !right.is_empty() {
                    for x in right {
                        inorder.push(x);
                    }
                }
                inorder
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
        assert_eq!(Solution::inorder_traversal(root), inorder);
    }
}
