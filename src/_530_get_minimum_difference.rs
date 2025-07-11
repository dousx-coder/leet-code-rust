use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [530. 二叉搜索树的最小绝对差值](https://leetcode.cn/problems/minimum-absolute-difference-in-bst/)
///
struct Solution;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = Vec::new();
        let mut curr_node = root;
        let mut prev: Option<i32> = None;
        let mut min_abs = i32::MAX;
        while curr_node.is_some() || !stack.is_empty() {
            // 尽可能往左走，并把路径上的节点压入栈中
            while let Some(node_rc) = curr_node {
                stack.push(node_rc.clone());
                curr_node = node_rc.borrow().left.clone();
            }
            // 弹出栈顶节点，访问它并转向右子树
            if let Some(node_rc) = stack.pop() {
                let curr_val = node_rc.borrow().val;
                if let Some(prev_val) = prev {
                    let abs = (curr_val - prev_val).abs();
                    if abs < min_abs {
                        min_abs = abs;
                    }
                }
                prev = Some(curr_val);
                curr_node = node_rc.borrow().right.clone();
            }
        }
        min_abs
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let vec = vec![1, 2, 3, 4, 6];
        let bst = TreeNode::build_bst(&vec);
        assert_eq!(Solution::get_minimum_difference(bst), 1);
    }

    #[test]
    fn t2() {
        let vec = vec![0, 1, 12, 48, 49];
        let bst = TreeNode::build_bst(&vec);
        assert_eq!(Solution::get_minimum_difference(bst), 1);
    }
}
