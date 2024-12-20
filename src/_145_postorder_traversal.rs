use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

///
/// `145 二叉树的后序遍历`
///
/// https://leetcode.cn/problems/binary-tree-postorder-traversal/
///
struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut postorder = vec![];
                let left = Self::postorder_traversal(root.borrow().left.clone());
                if !left.is_empty() {
                    for x in left {
                        postorder.push(x);
                    }
                }
                let right = Self::postorder_traversal(root.borrow().right.clone());
                if !right.is_empty() {
                    for x in right {
                        postorder.push(x);
                    }
                }
                postorder.push(root.borrow().val);
                postorder
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
        assert_eq!(
            Solution::postorder_traversal(root),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
    }
}
