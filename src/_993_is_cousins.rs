use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
///
///[993. 二叉树的堂兄弟节点](https://leetcode.cn/problems/cousins-in-binary-tree/description/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        // 下一层
        let mut next_level_node = VecDeque::new();
        // 当前层
        let mut curr_level_node = VecDeque::new();
        curr_level_node.push_back(root.clone());
        let mut curr_level_val = vec![];
        loop {
            match curr_level_node.pop_front() {
                Some(node) => match node {
                    Some(rc) => {
                        curr_level_val.push(rc.borrow().val);
                        if curr_level_val.contains(&x) && curr_level_val.contains(&y) {
                            return true;
                        }
                        let left = rc.borrow().left.clone();
                        let right = rc.borrow().right.clone();
                        if left.is_some() && right.is_some() {
                            let left = left.unwrap().borrow().val;
                            let right = right.unwrap().borrow().val;
                            if (left == x && right == y) || (left == y && right == x) {
                                return false;
                            }
                        }
                        next_level_node.push_back(rc.borrow().left.clone());
                        next_level_node.push_back(rc.borrow().right.clone());
                    }
                    None => {}
                },
                None => {}
            }
            if curr_level_node.is_empty() && !next_level_node.is_empty() {
                while let Some(rc) = next_level_node.pop_front() {
                    curr_level_node.push_back(rc);
                }
                if !curr_level_val.is_empty() {
                    curr_level_val = vec![];
                }
                continue;
            }
            if curr_level_node.is_empty() && next_level_node.is_empty() {
                return false;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![1, 2, 4, 3];
        let inorder = vec![4, 2, 1, 3];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert!(!Solution::is_cousins(root, 3, 4));
    }

    #[test]
    fn t2() {
        let preorder = vec![1, 2, 4, 3, 5];
        let inorder = vec![2, 4, 1, 3, 5];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert!(Solution::is_cousins(root, 5, 4));
    }

    #[test]
    fn t3() {
        let preorder = vec![1, 2, 4, 3];
        let inorder = vec![2, 4, 1, 3];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert!(!Solution::is_cousins(root, 2, 3));
    }
}
