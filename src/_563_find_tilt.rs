use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [563. 二叉树的坡度](https://leetcode.cn/problems/binary-tree-tilt/)
///
struct Solution;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tilt = vec![];
        Self::tree_sum(root, &mut tilt);
        tilt.into_iter().sum()
    }
    fn tree_sum(root: Option<Rc<RefCell<TreeNode>>>, tilt: &mut Vec<i32>) -> i32 {
        match root {
            None => 0,
            Some(rc) => {
                let borrow = rc.borrow();
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                if left.is_none() && right.is_none() {
                    return borrow.val;
                }
                let left_sum = Self::tree_sum(left, tilt);
                let right_sum = Self::tree_sum(right, tilt);
                tilt.push((left_sum - right_sum).abs());
                borrow.val + left_sum + right_sum
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let sequential = vec![-1, 1, 2, 3];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::find_tilt(root), 1);
    }

    #[test]
    fn t2() {
        let sequential = vec![-1, 4, 2, 9, 3, 5, i32::MIN, 7];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::find_tilt(root), 15);
    }

    #[test]
    fn t3() {
        let sequential = vec![-1, 21, 7, 14, 1, 1, 2, 2, 3, 3];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::find_tilt(root), 9);
    }
}
