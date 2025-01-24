use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
///
/// 113 路径总和Ⅱ
///
/// https://leetcode.cn/problems/path-sum-ii/description/?envType=problem-list-v2&envId=binary-tree
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut curr = vec![];
        Self::dfs(root, target_sum, &mut ans, &mut curr);
        ans
    }
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        ans: &mut Vec<Vec<i32>>,
        currr: &mut Vec<i32>,
    ) {
        match root {
            None => {}
            Some(rc) => {
                let borrow = rc.borrow();
                if borrow.left.is_none() && borrow.right.is_none() && borrow.val == target_sum {
                    currr.push(borrow.val);
                    ans.push(currr.clone());
                    currr.pop();
                    return;
                }
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                let next_sum = target_sum - borrow.val;
                if left.is_some() {
                    currr.push(borrow.val);
                    Self::dfs(left, next_sum, ans, currr);
                    currr.pop();
                }
                if right.is_some() {
                    currr.push(borrow.val);
                    Self::dfs(right, next_sum, ans, currr);
                    currr.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![1, 2, 3];
        let inorder = vec![2, 1, 3];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::path_sum(root, 5);
        assert!(ans.is_empty());
    }
}
