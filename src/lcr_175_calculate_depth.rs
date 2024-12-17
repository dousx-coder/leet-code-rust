use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
///
/// `计算二叉树的深度`
///
/// https://leetcode.cn/problems/er-cha-shu-de-shen-du-lcof/description/
///
struct Solution;
impl Solution {
    pub fn calculate_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_rc) = root {
            let left = root_rc.borrow_mut().left.take();
            let right = root_rc.borrow_mut().right.take();
            let left = Self::calculate_depth(left);
            let right = Self::calculate_depth(right);
            return max(left, right) + 1;
        }
        0
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
}
