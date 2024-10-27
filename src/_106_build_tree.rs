///
/// https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/?envType=problem-list-v2&envId=hash-table
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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = inorder.len();
        Solution::build(&inorder, &postorder, 0, n, 0, n)
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
    fn build(
        inorder: &Vec<i32>,
        postorder: &Vec<i32>,
        post_begin: usize,
        post_end: usize,
        in_begin: usize,
        in_end: usize) -> Option<Rc<RefCell<TreeNode>>> {
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
        // 左中序区间，左闭右开[left_inorder_begin, left_inorder_end)
        let left_inorder_begin = in_begin;
        let left_inorder_end = delimiter_index;
        // 右中序区间，左闭右开[right_inorder_begin, right_inorder_end)
        let right_inorder_begin = delimiter_index + 1;
        let right_inorder_end = in_end;

        // 切割后序数组
        // 左后序区间，左闭右开[left_postorder_begin, left_postorder_end)
        let left_postorder_begin = post_begin;
        // 终止位置是 需要加上 中序区间的大小size
        let left_postorder_end = post_begin + delimiter_index - in_begin;
        // 右后序区间，左闭右开[right_postorder_begin, right_postorder_end)
        let right_postorder_begin = post_begin + (delimiter_index - in_begin);
        root_node.left = Solution::build(inorder, postorder,
                                         left_postorder_begin, left_postorder_end,
                                         left_inorder_begin, left_inorder_end);
        root_node.right = if post_end > 1 {
            // 排除最后一个元素，已经作为节点了
            let right_postorder_end = post_end - 1;
            Solution::build(inorder, postorder,
                            right_postorder_begin, right_postorder_end,
                            right_inorder_begin, right_inorder_end)
        } else { None };

        Some(Rc::new(RefCell::new(root_node)))
    }
}
#[cfg(test)]
mod tests {
    use std::path::Prefix;
    use super::*;
    fn tree_to_vec(ergodic_type: usize, vec: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
        match node {
            Some(node) => {
                if ergodic_type == 1 {
                    vec.push(node.borrow().val);
                    tree_to_vec(ergodic_type, vec, &node.borrow().left);
                    tree_to_vec(ergodic_type, vec, &node.borrow().right);
                } else if ergodic_type == 2 {
                    tree_to_vec(ergodic_type, vec, &node.borrow().left);
                    vec.push(node.borrow().val);

                    tree_to_vec(ergodic_type, vec, &node.borrow().right);
                } else {
                    tree_to_vec(ergodic_type, vec, &node.borrow().left);
                    tree_to_vec(ergodic_type, vec, &node.borrow().right);
                    vec.push(node.borrow().val);
                }
            }
            None => {}
        }
    }

    #[test]
    fn t1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let tree_root = Solution::build_tree(inorder.clone(), postorder.clone());
        {
            let mut pre_result = vec![];
            tree_to_vec(1, &mut pre_result, &tree_root);
            println!("{:?}", pre_result);
        }
        {
            let mut in_result = vec![];
            tree_to_vec(2, &mut in_result, &tree_root);
            println!("{:?}", in_result);
            assert_eq!(in_result, inorder);
        }
        {
            let mut post_result = vec![];
            tree_to_vec(3, &mut post_result, &tree_root);
            assert_eq!(post_result, postorder);
            println!("{:?}", post_result);
        }
    }
}
