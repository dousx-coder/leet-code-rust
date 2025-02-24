use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
///
/// 114 二叉树展开为链表
///
/// https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/?envType=problem-list-v2&envId=binary-tree
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let rc = root.clone().unwrap();
        if rc.borrow().left.is_none() && rc.borrow().right.is_none() {
            return;
        }
        // 使用栈先序遍历
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        stack.push(rc);
        let mut dummy = TreeNode::new(-1);
        let mut pre_node = None;
        while let Some(node) = stack.pop() {
            if dummy.right.is_none() {
                dummy.right = Some(node.clone());
                pre_node = dummy.right.clone();
            }
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let new_some = Some(node);
            pre_node.as_mut().unwrap().borrow_mut().right = new_some.clone();
            pre_node.as_mut().unwrap().borrow_mut().left = None;
            pre_node = new_some;
            // 先压右子树，再压左子树，确保左子树优先被访问
            if let Some(right) = right.clone() {
                stack.push(right);
            }
            if let Some(left) = left.clone() {
                stack.push(left);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![1, 2, 3, 4, 5, 6];
        let inorder = vec![3, 2, 4, 1, 5, 6];
        let mut root = TreeNode::build_binary_tree(&preorder, &inorder);
        Solution::flatten(&mut root);
        let preorder = TreeNode::preorder_traversal_recursive(&mut root);
        assert_eq!(preorder, vec![1, 2, 3, 4, 5, 6]);
        let postorder = TreeNode::postorder_traversal_recursive(&mut root);
        assert_eq!(postorder, vec![6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn t2() {
        let mut root = None;
        Solution::flatten(&mut root);
    }

    #[test]
    fn t3() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        Solution::flatten(&mut root);
        let preorder = TreeNode::preorder_traversal_recursive(&mut root);
        assert_eq!(preorder, vec![1]);
        let postorder = TreeNode::postorder_traversal_recursive(&mut root);
        assert_eq!(postorder, vec![1]);
    }
}
