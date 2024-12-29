use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

///
/// 958 二叉树的完全性检验
///
/// https://leetcode.cn/problems/check-completeness-of-a-binary-tree/
struct Solution;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut sequential = Vec::new();
        sequential.push((root, 1));
        let mut i = 0;
        while i < sequential.len() {
            if let Some(mut x) = sequential.get(i) {
                let node = x.0.clone();
                let index = x.1;
                if let Some(rc) = node {
                    sequential.push((rc.borrow().left.clone(), 2 * index));
                    sequential.push((rc.borrow().right.clone(), 2 * index + 1));
                }
            }
            i += 1;
        }
        if let Some(x) = sequential.get(i - 1) {
            let index = x.1;
            index == sequential.len()
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
        let sequential = vec![-1, 1, 2, 3, 4, 5, 6];
        let root = TreeNode::build_complete_tree(&sequential);
        assert!(Solution::is_complete_tree(root));
    }

    #[test]
    fn t2() {
        let sequential = vec![-1, 1, 2, 3, 4, 5, i32::MIN, 7];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert!(!Solution::is_complete_tree(root));
    }

    #[test]
    fn t3() {
        let sequential = vec![-1, 1, 2, 3, 5, i32::MIN, 7, 8];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::is_complete_tree(root), false);
    }

    #[test]
    fn t4() {
        let sequential = vec![-1, 1, 2];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert!(Solution::is_complete_tree(root));
    }

    #[test]
    fn t5() {
        let sequential = vec![-1, 1, 2, 3, 5];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert!(Solution::is_complete_tree(root));
    }
}
