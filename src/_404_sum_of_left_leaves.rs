use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [404. 左叶子之和](https://leetcode.cn/problems/sum-of-left-leaves/)
///
struct Solution;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::left_leaves_sum(root, false)
    }
    fn left_leaves_sum(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match node {
            None => 0,
            Some(rc) => {
                let borrow = rc.borrow();
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                if left.is_none() && right.is_none() && is_left {
                    return borrow.val;
                }
                Self::left_leaves_sum(left, true) + Self::left_leaves_sum(right, false)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let sequential = vec![-1, 3, 9, 20, i32::MIN, i32::MIN, 15, 7];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::sum_of_left_leaves(root), 24);
    }

    #[test]
    fn t2() {
        let sequential = vec![-1, 1];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::sum_of_left_leaves(root), 0);
    }

    #[test]
    fn t3() {
        let sequential = vec![-1, 1, 2, 3, 4, 5, 6, 7, 8];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::sum_of_left_leaves(root), 14);
    }
}
