use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl TreeNode {
    ///
    /// [`preorder`]前缀
    ///
    /// [`inorder`]中缀
    pub fn build_binary_tree(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        if preorder.len() != inorder.len() {
            panic!()
        }
        let n = preorder.len();
        Self::build_by_preorder_inorder(preorder, inorder, 0, n - 1, 0, n - 1)
    }
    fn build_by_preorder_inorder(
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
        node.right = Self::build_by_preorder_inorder(
            preorder,
            inorder,
            pre_left + sub_left_len + 1,
            pre_right,
            in_root_idx + 1,
            in_right,
        );
        node.left = if in_root_idx >= 1 {
            // in_root_idx - 1 usize 可能溢出(负数)
            Self::build_by_preorder_inorder(
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
