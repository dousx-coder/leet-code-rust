use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
///
/// [513. 找树左下角的值](https://leetcode.cn/problems/find-bottom-left-tree-value/)
///
struct Solution;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::recursive_solution(root)
    }

    ///层序遍历解法
    fn level_order_solution(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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

    /// 递归解法
    fn recursive_solution(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root.clone(), 1).1
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, dept: i32) -> (i32, i32) {
        match root {
            None => (i32::MIN, i32::MIN),
            Some(rc) => {
                let borrow = rc.borrow();
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                if left.is_none() && right.is_none() {
                    return (dept + 1, borrow.val);
                }
                let left_dfs = Self::dfs(left, dept + 1);
                let right_dfs = Self::dfs(right, dept + 1);
                let left_dept = left_dfs.0;
                let right_dept = right_dfs.0;
                let left_val = left_dfs.1;
                let right_val = right_dfs.1;
                if left_dept >= right_dept {
                    (left_dept + 1, left_val)
                } else {
                    (right_dept + 1, right_val)
                }
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
    #[test]
    fn t3() {
        let preorder = vec![1, 2, 4, 3, 5, 7, 6];
        let inorder = vec![4, 2, 1, 7, 5, 3, 6];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::level_order_solution(root), 7);
    }
    #[test]
    fn t4() {
        let preorder = vec![1, 2, 4, 3, 5, 7, 6];
        let inorder = vec![4, 2, 1, 7, 5, 3, 6];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(Solution::recursive_solution(root), 7);
    }
    #[test]
    fn t5() {
        let sequential = vec![-1, 2, 1, 3];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        assert_eq!(Solution::recursive_solution(root), 1);
    }
}
