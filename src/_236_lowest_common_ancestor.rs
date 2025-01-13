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
        Self::postorder(root, p?.borrow().val, q?.borrow().val)
    }
    fn postorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(rc) => {
                let borrow = rc.borrow();
                let left = Self::postorder(borrow.left.clone(), p, q);
                let right = Self::postorder(borrow.right.clone(), p, q);
                let val = borrow.val;
                if left.is_some() && right.is_some() {
                    let left = left.unwrap();
                    let right = right.unwrap();
                    let left_val = left.borrow().val;
                    let right_val = right.borrow().val;
                    if left_val == p && right_val == q {
                        return Some(rc.clone());
                    }
                    if left_val == q || right_val == p {
                        return Some(rc.clone());
                    }
                    return None;
                };
                if left.is_some() {
                    let l = left?;
                    let left_val = l.borrow().val;
                    if left_val == p && val == q {
                        return Some(rc.clone());
                    }
                    if left_val == q && val == p {
                        return Some(rc.clone());
                    }
                    return Some(l);
                };
                if right.is_some() {
                    let r = right?;
                    let right_val = r.borrow().val;
                    if right_val == p && val == q {
                        return Some(rc.clone());
                    }
                    if right_val == q && val == p {
                        return Some(rc.clone());
                    }
                    return Some(r);
                };

                if val == p || val == q {
                    return Some(rc.clone());
                }
                None
            }
        }
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
        let p = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let ans = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 5);
    }
    #[test]
    fn t2() {
        let preorder = vec![1, 2];
        let inorder = vec![2, 1];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::postorder(root.clone(), 1, 2);
        assert_eq!(ans.unwrap().as_ref().borrow().val, 1);
    }
}
