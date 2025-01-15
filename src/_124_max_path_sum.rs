struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

///
/// 124 二叉树中最大路径总和
///
/// 二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。
///
/// 同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定经过根节点。
///
/// 路径和 是路径中各节点值的总和。
///
/// https://leetcode.cn/problems/binary-tree-maximum-path-sum/?envType=problem-list-v2&envId=binary-tree
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::dfs(root, &mut max_sum);
        max_sum
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(rc) => {
                let borrow = rc.borrow();
                let val = borrow.val;
                if val > *max_sum {
                    *max_sum = val;
                }
                if borrow.left.is_none() && borrow.right.is_none() {
                    return val;
                }
                let left = Self::dfs(borrow.left.clone(), max_sum);
                let right = Self::dfs(borrow.right.clone(), max_sum);
                let left_sum = left + val;
                let right_sum = right + val;
                let curr_sum = left + val + right;
                let curr_sum = max(max(val, left_sum), max(left + val + right, right_sum));
                if curr_sum > *max_sum {
                    *max_sum = curr_sum;
                };
                max(max(val, left_sum), max(val, right_sum))
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
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 6);
    }
    #[test]
    fn t2() {
        let preorder = vec![9, -10, 20, 15, 7];
        let inorder = vec![9, -10, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 42);
    }

    #[test]
    fn t3() {
        let preorder = vec![9, 1, 20, 15, 7];
        let inorder = vec![9, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 45);
    }
    #[test]
    fn t4() {
        let preorder = vec![10];
        let inorder = vec![10];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 10);
    }
    #[test]
    fn t5() {
        let preorder = vec![2, -1];
        let inorder = vec![-1, 2];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 2);
    }
    #[test]
    fn t6() {
        let preorder = vec![-2, 1];
        let inorder = vec![1, -2];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 1);
    }

    #[test]
    fn t7() {
        let preorder = vec![1, -2, 3];
        let inorder = vec![-2, 1, 3];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 4);
    }

    #[test]
    fn t8() {
        // 坑点，这里有两个2 不能用前序和后序构建树，
        let m = i32::MIN;
        let sequential = vec![
            vec![9],
            vec![6, -3],
            vec![m, m, -6, 2],
            vec![m, m, m, m, m, m, 2, m],
            vec![m, m, m, m, m, m, m, m, m, m, m, m, -6, -6],
            vec![
                m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, m, -6,
            ],
        ];
        //    9
        //   /  \
        //  6    -3
        //      /   \
        //    -6     2
        //          /
        //         2
        //       /   \
        //      -6    -6
        //     /
        //    -6
        let root = TreeNode::build_tree_by_two_sequential(sequential, true);
        let ans = Solution::max_path_sum(root);
        assert_eq!(ans, 16);
    }
}
