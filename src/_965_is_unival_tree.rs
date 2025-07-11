use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
///
/// [965. 单值二叉树](https://leetcode.cn/problems/univalued-binary-tree/description/)
///
struct Solution;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut level_traversal = vec![];
        // 等级
        // 下一层
        let mut next_level = VecDeque::new();
        // 当前层
        let mut curr_level = VecDeque::new();
        curr_level.push_back(root.clone());
        loop {
            match curr_level.pop_front() {
                Some(node) => match node {
                    Some(rc) => {
                        let v = rc.borrow().val;
                        if !level_traversal.is_empty() && v != level_traversal[0] {
                            return false;
                        }
                        level_traversal.push(v);
                        next_level.push_back(rc.borrow().left.clone());
                        next_level.push_back(rc.borrow().right.clone());
                    }
                    None => {}
                },
                None => {}
            }
            if curr_level.is_empty() && !next_level.is_empty() {
                while let Some(rc) = next_level.pop_front() {
                    curr_level.push_back(rc);
                }
                continue;
            }
            if curr_level.is_empty() && next_level.is_empty() {
                break;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let root = TreeNode::build_tree_by_sequential_storage(&vec![-1, 1, 2, 3, 4, 5], false);
        assert_eq!(Solution::is_unival_tree(root), false);
    }

    #[test]
    fn t2() {
        let root = TreeNode::build_tree_by_sequential_storage(&vec![-1, 1, 1, 1, 1, 1], false);
        assert_eq!(Solution::is_unival_tree(root), true);
    }
}
