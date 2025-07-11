use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

///
/// [654. 最大二叉树](https://leetcode.cn/problems/maximum-binary-tree/)
///
struct Solution;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&nums, 0, nums.len())
    }
    fn build(nums: &Vec<i32>, begin: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if begin >= end {
            return None;
        }
        let mut max_val = i32::MIN;
        let mut max_index = begin;
        for i in begin..end {
            if nums[i] > max_val {
                max_val = nums[i];
                max_index = i;
            }
        }
        let mut root_node = TreeNode::new(max_val);
        root_node.left = Self::build(nums, begin, max_index);
        root_node.right = Self::build(nums, max_index + 1, end);
        Some(Rc::new(RefCell::new(root_node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let root = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
        let preorder = TreeNode::preorder_traversal_recursive(&root);
        assert_eq!(preorder, vec![6, 3, 2, 1, 5, 0]);
        let inorder = TreeNode::inorder_traversal_recursive(&root);
        assert_eq!(inorder, vec![3, 2, 1, 6, 0, 5]);
    }

    #[test]
    fn t2() {
        let root = Solution::construct_maximum_binary_tree(vec![3, 2, 1]);
        let preorder = TreeNode::preorder_traversal_recursive(&root);
        assert_eq!(preorder, vec![3, 2, 1]);
        let inorder = TreeNode::inorder_traversal_recursive(&root);
        assert_eq!(inorder, vec![3, 2, 1]);
    }
}
