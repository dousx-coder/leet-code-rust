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
    ///  //     3
    ///  //    / \
    ///  //   9   20
    ///  //  /    / \
    ///  // 6    15  7
    ///  //     /
    ///  //    1
    ///  use leet_code_rust::common::binary_tree::TreeNode;
    ///  let preorder = vec![3, 9, 6, 20, 15, 1, 7];
    ///  let inorder = vec![6, 9, 3, 1, 15, 20, 7];
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

    /// 根据 顺序储存 构建
    ///
    /// 最后一层的叶子节点的孩子可不写
    /// # 参数
    /// - [`sequential`]: 下标从1开始
    ///
    /// # 返回值
    ///
    /// [`TreeNode`]根节点
    ///
    /// # 示例
    /// ```
    ///  //     3
    ///  //    / \
    ///  //   9   20
    ///  //  /    / \
    ///  // 6    15  7
    ///  //     /
    ///  //    1
    ///  use leet_code_rust::common::binary_tree::TreeNode;
    ///  let sequential = vec![
    ///            None,
    ///            Some(3),
    ///            Some(9), Some(20),
    ///            Some(6), None, Some(15), Some(7),
    ///            None, None, None, None, Some(1), None, None, None, None,
    ///        ];
    ///        let root = TreeNode::build_by_sequential_storage(&sequential);
    ///        assert_eq!(
    ///           TreeNode::postorder_traversal_iter(root.clone()),
    ///           vec![6, 9, 1, 15, 7, 20, 3]
    ///       );
    ///
    /// ```
    pub fn build_by_sequential_storage(
        sequential: &Vec<Option<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_by_sequential_storage_recursive(sequential, 1)
    }
    fn build_by_sequential_storage_recursive(
        sequential: &Vec<Option<i32>>,
        index: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if index >= sequential.len() {
            return None;
        }
        // 当前节点的node
        match sequential[index] {
            None => None,
            Some(val) => {
                let mut node = TreeNode::new(val);
                node.left = Self::build_by_sequential_storage_recursive(sequential, 2 * index);
                node.right = Self::build_by_sequential_storage_recursive(sequential, 2 * index + 1);
                Some(Rc::new(RefCell::new(node)))
            }
        }
    }

    /// 前序遍历二叉树(递归)
    pub fn preorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut preorder = vec![root.borrow().val];
                let left = Self::preorder_traversal_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        preorder.push(x);
                    }
                }
                let right = Self::preorder_traversal_recursive(&root.borrow().right);
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
    pub fn inorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut inorder = vec![];
                let left = Self::inorder_traversal_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        inorder.push(x);
                    }
                }
                inorder.push(root.borrow().val);
                let right = Self::inorder_traversal_recursive(&root.borrow().right);
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
    pub fn postorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut postorder = vec![];
                let left = Self::postorder_traversal_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        postorder.push(x);
                    }
                }
                let right = Self::postorder_traversal_recursive(&root.borrow().right);
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
    ///  层次遍历(迭代)
    pub fn hierarchical_traversal_iter(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut hierarchical_traversal = vec![];
        // 下一层
        let mut next_hierarchical = VecDeque::new();
        // 当前层
        let mut curr_hierarchical = VecDeque::new();
        curr_hierarchical.push_back(root.clone());
        loop {
            match curr_hierarchical.pop_front() {
                Some(node) => match node {
                    Some(rc) => {
                        hierarchical_traversal.push(rc.borrow().val);
                        next_hierarchical.push_back(rc.borrow().left.clone());
                        next_hierarchical.push_back(rc.borrow().right.clone());
                    }
                    None => {}
                },
                None => {}
            }
            if curr_hierarchical.is_empty() && !next_hierarchical.is_empty() {
                while let Some(rc) = next_hierarchical.pop_front() {
                    curr_hierarchical.push_back(rc);
                }
                continue;
            }
            if curr_hierarchical.is_empty() && next_hierarchical.is_empty() {
                break;
            }
        }
        hierarchical_traversal
    }

    /// 前序遍历二叉树(非递归) 迭代
    pub fn preorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut preorder = vec![];
        // 非递归和递归思路一样都是利用栈实现
        // rust语言提供的递归是利用栈实现的，直接使用递归函数，节省了开发者的编码且易于理解。
        // 非递归是用队列自定义栈，模拟rust语言的递归遍历过程
        let mut deque = VecDeque::new();
        deque.push_back(root);
        while !deque.is_empty() {
            match deque.pop_back() {
                Some(node) => match node {
                    Some(rc) => {
                        preorder.push(rc.borrow().val);
                        let right = rc.borrow().right.clone();
                        deque.push_back(right);
                        let left = rc.borrow().left.clone();
                        deque.push_back(left);
                    }
                    None => {}
                },
                None => {}
            }
        }
        preorder
    }

    /// 后序遍历二叉树(非递归) 迭代
    pub fn postorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut deque = VecDeque::new();
        deque.push_back(root);
        while !deque.is_empty() {
            match deque.pop_back() {
                Some(node) => match node {
                    Some(rc) => {
                        result.push(rc.borrow().val);
                        // 前序遍历是根左右，后序遍历是左右根
                        // 先将left入栈(和先序入栈顺序不一样),最终得到的数组再反转
                        let left = rc.borrow().left.clone();
                        deque.push_back(left);
                        let right = rc.borrow().right.clone();
                        deque.push_back(right);
                    }
                    None => {}
                },
                None => {}
            }
        }
        result.reverse();
        result
    }

    /// 中序遍历二叉树(非递归) 迭代
    pub fn inorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut inorder = vec![];
        // ec 替代 VecDeque,Vec 更适合后进先出的需求
        let mut stack = Vec::new();
        let mut current = root;

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

        inorder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::preorder_traversal_recursive(&root), preorder);
        assert_eq!(TreeNode::inorder_traversal_recursive(&root), inorder);
        assert_eq!(
            TreeNode::postorder_traversal_recursive(&root),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
        assert_eq!(
            TreeNode::hierarchical_traversal_iter(&root),
            vec![3, 9, 20, 6, 15, 7, 1]
        );
    }

    #[test]
    fn t2() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::preorder_traversal_iter(root.clone()), preorder);
    }

    #[test]
    fn t3() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(
            TreeNode::postorder_traversal_iter(root.clone()),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
    }

    #[test]
    fn t4() {
        let sequential = vec![
            None,
            Some(3),
            Some(9),
            Some(20),
            Some(6),
            None,
            Some(15),
            Some(7),
            None,
            None,
            None,
            None,
            Some(1),
            None,
            None,
            None,
            None,
        ];
        let root = TreeNode::build_by_sequential_storage(&sequential);
        assert_eq!(
            TreeNode::postorder_traversal_iter(root.clone()),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
    }

    #[test]
    fn t5() {
        let sequential = vec![
            i32::MIN,
            3,
            9,
            20,
            6,
            i32::MIN,
            15,
            7,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            1,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
        ];
        let sequential = sequential
            .iter()
            .map(|x| if *x == i32::MIN { None } else { Some(*x) })
            .collect::<Vec<Option<i32>>>();
        let root = TreeNode::build_by_sequential_storage(&sequential);
        assert_eq!(
            TreeNode::postorder_traversal_iter(root.clone()),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
    }

    #[test]
    fn t6() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::inorder_traversal_iter(root.clone()), inorder);
    }

    #[test]
    fn t7() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::inorder_traversal_iter(root.clone()), inorder);
    }

    #[test]
    fn t8() {
        let preorder = vec![5, 4, 1, 2, 6];
        let inorder = vec![1, 4, 2, 5, 6];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::inorder_traversal_iter(root.clone()), inorder);
    }
}
