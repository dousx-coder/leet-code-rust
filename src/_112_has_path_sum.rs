use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
/// 112 路径总和
///
/// https://leetcode.cn/problems/path-sum/
struct Solution;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(rc) => {
                let borrow = rc.borrow();
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                if left.is_none() && right.is_none() && borrow.val == target_sum {
                    return true;
                }
                let next_target_num = target_sum - borrow.val;
                Self::has_path_sum(left, next_target_num)
                    || Self::has_path_sum(right, next_target_num)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![5, 4, 11, 7, 2, 8, 13, 4, 1];
        let inorder = vec![7, 11, 2, 4, 5, 13, 8, 4, 1];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert!(Solution::has_path_sum(root, 22));
    }

    #[test]
    fn t2() {
        let sequential = vec![-1, 1, 2, 3];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::has_path_sum(root, 5), false);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::has_path_sum(None, 0), false);
    }
}
