use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
///
/// [199. 二叉树的右视图](https://leetcode.cn/problems/binary-tree-right-side-view/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    /// 层序遍历保留每一层最后一个
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut level_order_last = vec![];
        // 下一层
        let mut next_level_node = VecDeque::new();
        // 当前层
        let mut curr_level_node = VecDeque::new();
        curr_level_node.push_back(root.clone());
        let mut curr_level = vec![];
        loop {
            match curr_level_node.pop_front() {
                Some(node) => match node {
                    Some(rc) => {
                        curr_level.push(rc.borrow().val);
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
                if !curr_level.is_empty() {
                    Self::push_last(&mut level_order_last, &curr_level);
                    curr_level = vec![];
                }
                continue;
            }
            if curr_level_node.is_empty() && next_level_node.is_empty() {
                if !curr_level.is_empty() {
                    Self::push_last(&mut level_order_last, &curr_level);
                }
                return level_order_last;
            }
        }
    }

    fn push_last(level_order_last: &mut Vec<i32>, curr_level: &Vec<i32>) {
        if let Some(rc) = curr_level.last() {
            level_order_last.push(rc.clone());
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![1, 2, 5, 3, 4];
        let inorder = vec![2, 5, 1, 3, 4];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }
}
