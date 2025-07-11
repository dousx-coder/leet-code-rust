use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [437. 路径总和Ⅲ](https://leetcode.cn/problems/path-sum-iii/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = Self::node_sum(root.clone(), target_sum as i64);
        let rc = root.unwrap();
        let borrow = rc.borrow();
        let left = borrow.left.clone();
        let right = borrow.right.clone();
        ans += Self::path_sum(left.clone(), target_sum);
        ans += Self::path_sum(right.clone(), target_sum);
        ans
    }
    fn node_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32 {
        match root {
            None => 0,
            Some(rc) => {
                let borrow = rc.borrow();
                let next_target = target_sum - borrow.val as i64;
                let mut ans = if next_target == 0 { 1 } else { 0 };
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                ans += Self::node_sum(left, next_target);
                ans += Self::node_sum(right, next_target);
                ans
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let sequential = vec![
            -1,
            1000000000,
            1000000000,
            i32::MIN,
            294967296,
            i32::MIN,
            1000000000,
            i32::MIN,
            1000000000,
            i32::MIN,
            1000000000,
        ];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        let ans = Solution::path_sum(root, 0);
        assert_eq!(ans, 0);
    }
}
