///
/// 538 把二叉搜索树转换为累加数
///
/// https://leetcode.cn/problems/convert-bst-to-greater-tree/
struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut curr_sum = 0;
        Self::convert(&root, &mut curr_sum)
    }
    pub fn convert(
        root: &Option<Rc<RefCell<TreeNode>>>,
        curr_sum: &mut i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(rc) => {
                let borrow = rc.borrow();
                let right = Self::convert(&borrow.right, curr_sum);
                let new_val = borrow.val + *curr_sum;
                *curr_sum = new_val;
                let left = Self::convert(&borrow.left, curr_sum);
                let mut new_node = TreeNode::new(new_val);
                new_node.left = left;
                new_node.right = right;
                Some(Rc::new(RefCell::new(new_node)))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![4, 1, 0, 2, 3, 6, 5, 7, 8];
        let inorder = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let tree = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::convert_bst(tree);
        let preorder = TreeNode::preorder_traversal_recursive(&ans);
        let inorder = TreeNode::inorder_traversal_recursive(&ans);
        assert_eq!(preorder, vec![30, 36, 36, 35, 33, 21, 26, 15, 8]);
        assert_eq!(inorder, vec![36, 36, 35, 33, 30, 26, 21, 15, 8]);
    }
}
