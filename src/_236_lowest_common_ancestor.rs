struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// 236 二叉树的最近公共祖先
///
/// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            // 这里如果直接用PartialEq比较 root和p ，则需要保证输入参数是树中的节点，而不能新创建一个TreeNode节点
            // 这种效率更高
            return root;
        }
        /*
        if root.is_none() {
            return root;
        }
        let root_val = root.clone()?.borrow().val;
        let p_val = p.clone()?.borrow().val;
        let q_val = q.clone()?.borrow().val;
        if root_val == p_val || root_val == q_val {
            return root;
        }
        */
        let x = root.as_ref()?;
        let left = Self::lowest_common_ancestor(x.borrow().left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(x.borrow().right.clone(), p, q);
        if left.is_some() && right.is_some() {
            return root;
        };
        if left.is_some() { left } else { right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![3, 5, 6, 2, 7, 4, 1, 0, 8];
        let inorder = vec![6, 5, 7, 2, 4, 3, 0, 1, 8];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let p = TreeNode::find_first_val(root.clone(), 6);
        let q = TreeNode::find_first_val(root.clone(), 4);
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 5);
    }
    #[test]
    fn t2() {
        let preorder = vec![1, 2];
        let inorder = vec![2, 1];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let p = root.clone();
        let q = root.as_ref().unwrap().borrow().left.clone();
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 1);
    }
    #[test]
    fn t3() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let p = TreeNode::find_first_val(root.clone(), 1);
        let q = TreeNode::find_first_val(root.clone(), 7);
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 20);

        let p = TreeNode::find_first_val(root.clone(), 15);
        let q = TreeNode::find_first_val(root.clone(), 7);
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 20);

        let p = TreeNode::find_first_val(root.clone(), 3);
        let q = TreeNode::find_first_val(root.clone(), 9);
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 3);
    }
}
