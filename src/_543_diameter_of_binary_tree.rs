struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
/// 543 二叉树的直径
///
/// https://leetcode.cn/problems/diameter-of-binary-tree/
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        let i = Self::dfs(root, &mut max);
        max
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, curr_max: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(rc) => {
                let mut mbr = rc.borrow_mut();
                let right = mbr.right.take();
                let left = mbr.left.take();
                if right.is_none() && left.is_none() {
                    return 0;
                }
                let val = mbr.val;
                let mut curr_sum = 0;
                let mut child_max = 0;
                if left.is_some() {
                    let left = Self::dfs(left, curr_max);
                    curr_sum += left + 1;
                    if left > child_max || child_max == 0 {
                        child_max = left + 1;
                    }
                }
                if right.is_some() {
                    let right = Self::dfs(right, curr_max);
                    curr_sum += right + 1;
                    if right >= child_max || child_max == 0 {
                        child_max = right + 1;
                    }
                }
                if curr_sum > *curr_max {
                    *curr_max = curr_sum;
                }
                child_max
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        //     1
        //    / \
        //   2   3
        //  / \
        // 4   5
        let preorder = vec![1, 2, 4, 5, 3];
        let inorder = vec![4, 2, 5, 1, 3];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::diameter_of_binary_tree(root);
        assert_eq!(ans, 3);
    }

    #[test]
    fn t2() {
        let preorder = vec![1, 2];
        let inorder = vec![2, 1];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::diameter_of_binary_tree(root);
        assert_eq!(ans, 1);
    }

    #[test]
    fn t3() {
        let preorder = vec![1];
        let inorder = vec![1];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::diameter_of_binary_tree(root);
        assert_eq!(ans, 0);
    }

    #[test]
    fn t4() {
        //     2 
        //    /
        //   3
        //  /
        // 1
        let preorder = vec![2, 3, 1];
        let inorder = vec![1, 3, 2];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::diameter_of_binary_tree(root);
        assert_eq!(ans, 2);
    }
}
