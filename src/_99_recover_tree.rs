use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;
///
/// [99. 恢复二叉搜索树](https://leetcode.cn/problems/recover-binary-search-tree/description/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::oh(root);
    }
    /// 暴力解法
    fn violence(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut inorder = vec![];
        // vec 替代 VecDeque,Vec 更适合后进先出的需求
        let mut stack = Vec::new();
        let mut current = root.clone();
        while current.is_some() || !stack.is_empty() {
            // 尽可能往左走，并把路径上的节点压入栈中
            while let Some(node_rc) = current {
                stack.push(node_rc.clone());
                current = node_rc.borrow().left.clone();
            }
            // 弹出栈顶节点，访问它并转向右子树
            if let Some(node_rc) = stack.pop() {
                inorder.push(node_rc.borrow().val);
                current = node_rc.borrow().right.clone();
            }
        }
        inorder.sort_by(|a, b| b.cmp(a));

        let mut current = root.clone();
        while current.is_some() || !stack.is_empty() {
            // 尽可能往左走，并把路径上的节点压入栈中
            while let Some(node_rc) = current {
                stack.push(node_rc.clone());
                current = node_rc.borrow().left.clone();
            }

            // 弹出栈顶节点，访问它并转向右子树
            if let Some(node_rc) = stack.pop() {
                node_rc.borrow_mut().val = inorder.pop().unwrap();
                current = node_rc.borrow().right.clone();
            }
        }
    }
    /// 空间复杂度O(h) h指的是树的高度
    fn oh(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut x = None;
        let mut y = None;
        let mut pred: Option<Rc<RefCell<TreeNode>>> = None;
        let mut stack = Vec::new();
        let mut curr = root.clone();
        while curr.is_some() || !stack.is_empty() {
            while let Some(curr_node) = curr {
                stack.push(curr_node.clone());
                curr = curr_node.borrow().left.clone();
            }
            if let Some(stack_pop) = stack.pop() {
                if let Some(pre_rc) = pred {
                    if stack_pop.borrow().val < pre_rc.borrow().val {
                        y = Some(stack_pop.clone());
                        if x.is_none() {
                            x = Some(pre_rc.clone());
                        }
                    }
                }
                pred = Some(stack_pop.clone());
                curr = stack_pop.borrow().right.clone();
            }
        }
        if let Some(nx) = x {
            if let Some(ny) = y {
                swap(&mut nx.borrow_mut().val, &mut ny.borrow_mut().val);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder = vec![1, 3, 2];
        let inorder = vec![3, 2, 1];
        let mut root = TreeNode::build_binary_tree(&preorder, &inorder);
        Solution::violence(&mut root);
        let inorder = TreeNode::inorder_traversal_recursive(&mut root);
        assert_eq!(inorder, vec![1, 2, 3]);
    }
    #[test]
    fn t2() {
        let preorder = vec![3, 1, 4, 2];
        let inorder = vec![1, 3, 2, 4];
        let mut root = TreeNode::build_binary_tree(&preorder, &inorder);
        Solution::oh(&mut root);
        let inorder = TreeNode::inorder_traversal_recursive(&mut root);
        assert_eq!(inorder, vec![1, 2, 3, 4]);
    }
}
