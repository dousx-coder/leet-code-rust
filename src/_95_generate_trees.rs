use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
///[95. 不同的二叉搜索树Ⅱ](https://leetcode.cn/problems/unique-binary-search-trees-ii/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n <= 0 {
            return vec![];
        }
        Self::solution(1, n)
    }
    fn solution(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }
        let mut res: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        for i in start..=end {
            let mut sub_left_tree = Self::solution(start, i - 1);
            let mut sub_right_tree = Self::solution(i + 1, end);
            let mut ref_right = &mut sub_right_tree;
            for left in sub_left_tree {
                for right in &mut *ref_right {
                    let mut node = TreeNode::new(i);
                    node.left = left.clone();
                    node.right = right.clone();
                    res.push(Some(Rc::new(RefCell::new(node))));
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    #[test]
    fn t1() {
        let mut ans = hashset! {};
        let solution = Solution::generate_trees(3);
        for node in solution {
            let preorder = TreeNode::preorder_traversal_recursive(&node);
            let inorder = TreeNode::inorder_traversal_recursive(&node);
            ans.insert((preorder, inorder));
        }
        // 期望
        let expect = hashset! {
        (vec![3, 1, 2], vec![1, 2, 3]),
        (vec![3, 2, 1], vec![1, 2, 3]),
        (vec![1, 2, 3], vec![1, 2, 3]),
        (vec![1, 3, 2],vec! [1, 2, 3]),
        (vec![2, 1, 3], vec![1, 2, 3])};
        assert_eq!(ans, expect);
    }
}
