struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
///
/// 235 二叉搜索树的最近公共祖先
///
/// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/description/
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_val = root.clone()?.borrow().val;
        let p_val = p.clone()?.borrow().val;
        let q_val = q.clone()?.borrow().val;
        let min = min(p_val, q_val);
        let max = max(p_val, q_val);
        Self::find(root.clone(), min, max)
    }

    fn find(
        root: Option<Rc<RefCell<TreeNode>>>,
        min: i32,
        max: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let val = root.as_ref()?.borrow().val;
        if val == min || val == max {
            return root;
        }
        let root_ref = root.as_ref()?;
        // 当前根节点，比最大值大，说明max应该在左子树
        let left = if val > max {
            Self::find(root_ref.borrow().left.clone(), min, max)
        } else {
            Self::find(root_ref.borrow().right.clone(), min, max)
        };
        let right = if val < min {
            Self::find(root_ref.borrow().right.clone(), min, max)
        } else {
            Self::find(root_ref.borrow().left.clone(), min, max)
        };
        if left.is_some() && right.is_some() {
            return if left == right { left } else { root };
        }
        if left.is_some() { left } else { right }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let root = TreeNode::build_bst(&nums);
        let p = TreeNode::find_first_val(root.clone(), -10);
        let q = TreeNode::find_first_val(root.clone(), 9);
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 0);
    }
    #[test]
    fn t2() {
        let nums = vec![-10, -3, 0, 5, 9];
        let root = TreeNode::build_bst(&nums);
        let p = TreeNode::find_first_val(root.clone(), 0);
        let q = TreeNode::find_first_val(root.clone(), 5);
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 0);
    }

    #[test]
    fn t3() {
        let preorder = vec![6, 2, 0, 4, 3, 5, 8, 7, 9];
        let inorder = vec![0, 2, 3, 4, 5, 6, 7, 8, 9];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let p = TreeNode::find_first_val(root.clone(), 2);
        let q = TreeNode::find_first_val(root.clone(), 4);
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 2);
    }
}
