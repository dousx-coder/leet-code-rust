use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
struct Solution;
///
/// 501 二叉搜索树中的众数
///
/// https://leetcode.cn/problems/find-mode-in-binary-search-tree/description/
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = Vec::new();
        let mut curr_node = root;
        let mut curr_num_count = 0;
        let mut pre_max_count = 0;
        let mut pre: Option<i32> = None;
        while curr_node.is_some() || !stack.is_empty() {
            // 尽可能往左走，并把路径上的节点压入栈中
            while let Some(node_rc) = curr_node {
                stack.push(node_rc.clone());
                curr_node = node_rc.borrow().left.clone();
            }
            // 弹出栈顶节点，访问它并转向右子树
            if let Some(node_rc) = stack.pop() {
                let curr_val = node_rc.borrow().val;
                match pre {
                    None => {
                        // 处理根节点
                        curr_num_count = 1;
                        ans.push(curr_val);
                    }
                    Some(prev_val) => {
                        if curr_val == prev_val {
                            curr_num_count += 1;
                            if curr_num_count == pre_max_count {
                                ans.push(curr_val);
                            }
                            if curr_num_count > pre_max_count {
                                ans.clear();
                                pre_max_count = curr_num_count;
                                ans.push(curr_val);
                            }
                        } else {
                            curr_num_count = 1;
                            pre_max_count = max(curr_num_count, pre_max_count);
                            if curr_num_count >= pre_max_count {
                                ans.push(curr_val);
                            }
                        }
                    }
                }
                pre = Some(curr_val);
                curr_node = node_rc.borrow().right.clone();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let vec = vec![1, 2, 3, 3, 4, 6];
        let bst = TreeNode::build_bst(&vec);
        assert_eq!(Solution::find_mode(bst), vec![3]);
    }
    #[test]
    fn t2() {
        let vec = vec![1];
        let bst = TreeNode::build_bst(&vec);
        assert_eq!(Solution::find_mode(bst), vec![1]);
    }
    #[test]
    fn t3() {
        let vec = vec![1, 2];
        let bst = TreeNode::build_bst(&vec);
        assert_eq!(Solution::find_mode(bst), vec![1, 2]);
    }
    #[test]
    fn t4() {
        let vec = vec![0, 0, 0, 0, 1, 1, 1, 1];
        let bst = TreeNode::build_bst(&vec);
        assert_eq!(Solution::find_mode(bst), vec![0, 1]);
    }

    #[test]
    fn t5() {
        let vec = vec![0, 0, 0, 1, 1, 1, 1];
        let bst = TreeNode::build_bst(&vec);
        assert_eq!(Solution::find_mode(bst), vec![1]);
    }
}
