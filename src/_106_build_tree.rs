///
/// 106 根据中序和后序遍历构建二叉树
///
/// https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/?envType=problem-list-v2&envId=hash-table
struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() != postorder.len() {
            panic!()
        }
        let n = inorder.len();
        Self::build(&inorder, &postorder, 0, n, 0, n)
    }
    ///
    /// 第一步：如果数组大小为零的话，说明是空节点了。
    ///
    /// 第二步：如果不为空，那么取后序数组最后一个元素作为节点元素。
    ///
    /// 第三步：找到后序数组最后一个元素在中序数组的位置，作为切割点
    ///
    /// 第四步：切割中序数组，切成中序左数组和中序右数组 （顺序别搞反了，一定是先切中序数组）
    ///
    /// 第五步：切割后序数组，切成后序左数组和后序右数组
    ///
    /// 第六步：递归处理左区间和右区间
    ///
    /// 采用左闭右开 ，可以减少 rust 类型溢出判断
    fn build(
        inorder: &Vec<i32>,
        postorder: &Vec<i32>,
        post_begin: usize,
        post_end: usize,
        in_begin: usize,
        in_end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if post_begin == post_end {
            return None;
        }
        // 取后序数组最后一个元素作为节点元素。
        let root_node_value = postorder[post_end - 1];
        let mut root_node = TreeNode::new(root_node_value);

        if post_end - post_begin == 1 {
            return Some(Rc::new(RefCell::new(root_node)));
        }

        let mut delimiter_index = in_begin;
        while delimiter_index <= in_end && root_node_value != inorder[delimiter_index] {
            delimiter_index += 1;
        }

        // 切割中序数组
        // 左中序区间，左闭右开[left_in_begin, left_in_end)
        let left_in_begin = in_begin;
        let left_in_end = delimiter_index;
        // 右中序区间，左闭右开[right_in_begin, right_in_end)
        let right_in_begin = delimiter_index + 1;
        let right_in_end = in_end;

        // 切割后序数组
        // 左后序区间，左闭右开[left_post_begin, left_post_end)
        let left_post_begin = post_begin;
        // 终止位置是 需要加上 中序区间的大小size
        let left_post_end = post_begin + delimiter_index - in_begin;
        // 右后序区间，左闭右开[right_post_begin, right_post_end)
        let right_post_begin = post_begin + (delimiter_index - in_begin);
        // 排除最后一个元素，已经作为节点了
        let right_post_end = post_end - 1;
        root_node.left = Self::build(
            inorder,
            postorder,
            left_post_begin,
            left_post_end,
            left_in_begin,
            left_in_end,
        );
        root_node.right = Self::build(
            inorder,
            postorder,
            right_post_begin,
            right_post_end,
            right_in_begin,
            right_in_end,
        );
        Some(Rc::new(RefCell::new(root_node)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let tree_root = Solution::build_tree(inorder.clone(), postorder.clone());
        {
            let pre_result = TreeNode::preorder_traversal_recursive(&tree_root);
            assert_eq!(pre_result, vec![3, 9, 20, 15, 7]);
        }
        {
            let in_result = TreeNode::inorder_traversal_recursive(&tree_root);
            assert_eq!(in_result, inorder);
        }
        {
            let post_result = TreeNode::postorder_traversal_iter(tree_root.clone());
            assert_eq!(post_result, postorder);
        }
    }
}
