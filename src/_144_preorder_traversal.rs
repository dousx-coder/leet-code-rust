use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// `144 二叉树前序遍历`
///
///
/// https://leetcode.cn/problems/binary-tree-preorder-traversal/description/
///
struct Solution;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut preorder = vec![root.borrow().val];
                let left = Self::preorder_traversal(root.borrow().left.clone());
                if !left.is_empty() {
                    for x in left {
                        preorder.push(x);
                    }
                }
                let right = Self::preorder_traversal(root.borrow().right.clone());
                if !right.is_empty() {
                    for x in right {
                        preorder.push(x);
                    }
                }
                preorder
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //         1
        //       /   \
        //      2     3
        //     / \     \
        //    4   5     8
        //       / \   /
        //      6   7 9
        let preorder = vec![1, 2, 4, 5, 6, 7, 3, 8, 9];
        let inorder = vec![4, 6, 5, 7, 2, 1, 3, 9, 8];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let vec = Solution::preorder_traversal(root);
        assert_eq!(preorder, vec)
    }
}
