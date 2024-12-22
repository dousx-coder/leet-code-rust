use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
///
/// `104 二叉树的 最大深度 `
///
/// https://leetcode.cn/problems/maximum-depth-of-binary-tree/description/
struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = Solution::max_depth(node.borrow().left.clone());
                let right = Solution::max_depth(node.borrow().right.clone());
                1 + max(left, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::max_depth(root);
        assert_eq!(ans, 4);
    }
}
