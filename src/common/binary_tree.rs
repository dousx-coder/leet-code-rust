use std::cell::RefCell;
use std::collections::VecDeque;
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

    ///
    /// 根据前缀和中缀构建二叉树
    ///
    /// # 参数
    /// - `preorder`: 前缀
    /// - `inorder`: 中缀
    ///
    /// # 返回值
    ///
    /// [`TreeNode`]根节点
    ///
    /// # 示例
    /// ```
    ///  //   3
    ///  //  / \
    ///  // 9  20
    ///  //    / \
    ///  //   15  7
    ///  use leet_code_rust::common::binary_tree::TreeNode;
    ///  let preorder = vec![3, 9, 20, 15, 7];
    ///  let inorder = vec![9, 3, 15, 20, 7];
    ///  let root = TreeNode::build_binary_tree(&preorder, &inorder);
    /// ```
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
        let mut node = Self::new(preorder[pre_left]);

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

    /// 前序遍历二叉树(递归)
    pub fn preorder_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut preorder = vec![root.borrow().val];
                let left = Self::preorder_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        preorder.push(x);
                    }
                }
                let right = Self::preorder_recursive(&root.borrow().right);
                if !right.is_empty() {
                    for x in right {
                        preorder.push(x);
                    }
                }
                preorder
            }
        }
    }
    /// 中序遍历二叉树(递归)
    pub fn inorder_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut inorder = vec![];
                let left = Self::inorder_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        inorder.push(x);
                    }
                }
                inorder.push(root.borrow().val);
                let right = Self::inorder_recursive(&root.borrow().right);
                if !right.is_empty() {
                    for x in right {
                        inorder.push(x);
                    }
                }
                inorder
            }
        }
    }

    /// 后序遍历二叉树(递归)
    pub fn postorder_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut postorder = vec![];
                let left = Self::postorder_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        postorder.push(x);
                    }
                }
                let right = Self::postorder_recursive(&root.borrow().right);
                if !right.is_empty() {
                    for x in right {
                        postorder.push(x);
                    }
                }
                postorder.push(root.borrow().val);
                postorder
            }
        }
    }

    /// 前序遍历二叉树(非递归)
    pub fn preorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut preorder = vec![];
        // 非递归和递归思路一样都是利用栈实现
        // rust语言提供的递归是利用栈实现的，直接使用递归函数，节省了开发者的编码且易于理解。
        // 非递归是用队列自定义栈，模拟rust语言的递归遍历过程
        let mut deque = VecDeque::new();
        deque.push_back(root);
        while !deque.is_empty() {
            match deque.pop_back() {
                None => {}
                Some(node) => match node {
                    None => {}
                    Some(rc) => {
                        preorder.push(rc.borrow().val);
                        let right = rc.borrow().right.clone();
                        deque.push_back(right);
                        let left = rc.borrow().left.clone();
                        deque.push_back(left);
                    }
                },
            }
        }
        preorder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::preorder_recursive(&root), preorder);
        assert_eq!(TreeNode::inorder_recursive(&root), inorder);
        assert_eq!(TreeNode::postorder_recursive(&root), vec![9, 15, 7, 20, 3]);
    }

    #[test]
    fn t2() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::preorder(root.clone()), preorder);
    }
}
