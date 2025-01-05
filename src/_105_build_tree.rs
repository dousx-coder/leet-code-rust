///
/// 105 根据前序和中序遍历构建二叉树
///
/// https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/?envType=problem-list-v2&envId=hash-table
///
struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() != inorder.len() {
            panic!()
        }
        let n = preorder.len();
        Self::build(&preorder, &inorder, 0, n - 1, 0, n - 1)
    }
    fn build(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        pre_left: usize,
        pre_right: usize,
        in_left: usize,
        in_right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_left > pre_right || in_left > in_right {
            return None;
        }
        // 从当前需要处理的 前序数组从获取，根结点值=> 前序数组左边界就是根结点值
        let mut node = TreeNode::new(preorder[pre_left]);

        // 从中序数组中，从左边界向右找，当前根节点在中序数组中的下标
        let mut in_root_idx = in_left;
        while in_root_idx <= in_right && inorder[in_root_idx] != preorder[pre_left] {
            in_root_idx += 1;
        }
        // 根据根结点在中序数组中的下标 和 当前中序数组的左边界 => 得到 左子树长度
        let mut sub_left_len = in_root_idx - in_left;
        node.right = Self::build(
            preorder,
            inorder,
            pre_left + sub_left_len + 1,
            pre_right,
            in_root_idx + 1,
            in_right,
        );
        node.left = if in_root_idx >= 1 {
            // in_root_idx - 1 usize 可能溢出(负数)
            Self::build(
                preorder,
                inorder,
                pre_left + 1,
                pre_left + sub_left_len,
                in_left,
                in_root_idx - 1,
            )
        } else {
            None
        };
        Some(Rc::new(RefCell::new(node)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let tree_root = Solution::build_tree(preorder.clone(), inorder.clone());
        {
            let pre_result = TreeNode::preorder_traversal_recursive(&tree_root);
            assert_eq!(pre_result, preorder);
        }

        {
            let in_result = TreeNode::inorder_traversal_recursive(&tree_root);
            assert_eq!(in_result, inorder);
        }
        {
            let post_result = TreeNode::postorder_traversal_iter(tree_root.clone());
            assert_eq!(post_result, vec![9, 15, 7, 20, 3]);
        }
    }
}
