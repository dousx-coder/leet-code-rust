use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// 700 二叉搜索树中的搜索
///
/// https://leetcode.cn/problems/search-in-a-binary-search-tree/description/
struct Solution;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::iter_solution(root, val)
    }
    /// 递归解法
    fn recursion_solution(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(rc) => {
                let borrow = rc.borrow();
                if borrow.val == val {
                    return Some(rc.clone());
                }
                if borrow.left.is_none() && borrow.right.is_none() {
                    return None;
                }
                if val < borrow.val {
                    return Self::recursion_solution(borrow.left.clone(), val);
                }
                Self::recursion_solution(borrow.right.clone(), val)
            }
        }
    }
    /// 迭代解法
    fn iter_solution(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root;
        while let Some(rc) = node {
            let borrow = rc.borrow();
            if borrow.val == val {
                return Some(rc.clone());
            }
            if val < borrow.val {
                node = borrow.left.clone();
                continue;
            }
            node = borrow.right.clone();
        }
        None
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        let preorder1 = vec![4, 2, 1, 3, 7];
        let inorder1 = vec![1, 2, 3, 4, 7];
        let tree = TreeNode::build_binary_tree(&preorder1, &inorder1);
        let option = Solution::search_bst(tree, 2);
        let preorder = TreeNode::preorder_traversal_recursive(&option);
        let inorder_ = TreeNode::inorder_traversal_recursive(&option);
        assert_eq!(preorder, vec![2, 1, 3]);
        assert_eq!(inorder_, vec![1, 2, 3]);
    }

    #[test]
    fn t2() {
        let preorder1 = vec![4, 2, 1, 3, 7];
        let inorder1 = vec![1, 2, 3, 4, 7];
        let tree = TreeNode::build_binary_tree(&preorder1, &inorder1);
        let option = Solution::search_bst(tree, 5);
        assert!(option.is_none());
    }
}
