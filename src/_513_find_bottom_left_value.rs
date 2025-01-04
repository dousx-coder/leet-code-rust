use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
///
/// 513 找树左下角的值
///
/// https://leetcode.cn/problems/find-bottom-left-tree-value/
struct Solution;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 下一层
        let mut next_level_node = VecDeque::new();
        // 当前层节点的值列表
        let mut curr_node_val = vec![];
        // 当前层
        let mut curr_level_node = VecDeque::new();
        curr_level_node.push_back(root.clone());
        loop {
            match curr_level_node.pop_front() {
                Some(node) => match node {
                    Some(rc) => {
                        curr_node_val.push(rc.borrow().val);
                        if rc.borrow().left.is_some() {
                            next_level_node.push_back(rc.borrow().left.clone());
                        }
                        if rc.borrow().right.is_some() {
                            next_level_node.push_back(rc.borrow().right.clone());
                        }
                    }
                    None => {}
                },
                None => {}
            }
            if curr_level_node.is_empty() && !next_level_node.is_empty() {
                while let Some(rc) = next_level_node.pop_front() {
                    curr_level_node.push_back(rc);
                }
                curr_node_val.clear();
                continue;
            }
            if curr_level_node.is_empty() && next_level_node.is_empty() {
                return curr_node_val[0];
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let sequential = vec![-1, 2, 1, 3];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::find_bottom_left_value(root), 1);
    }
    #[test]
    fn t2() {
        let preorder = vec![1, 2, 4, 3, 5, 7, 6];
        let inorder = vec![4, 2, 1, 7, 5, 3, 6];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::find_bottom_left_value(root), 7);
    }
}
