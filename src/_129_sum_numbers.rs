use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [129. 求根节点到叶节点数字之和](https://leetcode.cn/problems/sum-root-to-leaf-numbers/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums: Vec<i32> = vec![];
        Self::dfs(root, &mut vec![], &mut nums);
        if nums.is_empty() {
            0
        } else {
            nums.iter().sum()
        }
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, curr_path: &mut Vec<i32>, nums: &mut Vec<i32>) {
        match root {
            None => {}
            Some(rc) => {
                let mut borrow = rc.borrow_mut();
                let val = borrow.val;
                curr_path.push(val);
                let mut left = borrow.left.take();
                let mut right = borrow.right.take();
                if left.is_none() && right.is_none() {
                    let num = curr_path
                        .iter()
                        .map(|&x| x.to_string())
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    nums.push(num);
                    curr_path.pop();
                    return;
                }
                if left.is_some() {
                    Self::dfs(left, curr_path, nums);
                }
                if right.is_some() {
                    Self::dfs(right, curr_path, nums);
                }
                curr_path.pop();
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
        let ans = Solution::sum_numbers(root);
        assert_eq!(ans, 25);
    }

    #[test]
    fn t2() {
        let preorder = vec![4, 9, 5, 1, 0];
        let inorder = vec![5, 9, 1, 4, 0];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::sum_numbers(root);
        assert_eq!(ans, 1026);
    }
}
