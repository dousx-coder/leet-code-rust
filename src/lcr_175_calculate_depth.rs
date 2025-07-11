use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
///
/// [LCR 175计算二叉树的深度](https://leetcode.cn/problems/er-cha-shu-de-shen-du-lcof/description/)
///
struct Solution;
impl Solution {
    pub fn calculate_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let left = rc.borrow_mut().left.take();
                let right = rc.borrow_mut().right.take();
                let left = Self::calculate_depth(left);
                let right = Self::calculate_depth(right);
                max(left, right) + 1
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let ans = Solution::calculate_depth(root);
        assert_eq!(ans, 1);
    }

    #[test]
    fn t2() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::calculate_depth(root);
        assert_eq!(ans, 3);
    }
}
