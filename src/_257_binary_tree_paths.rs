///
/// 257  二叉树的所有路径
///
/// https://leetcode.cn/problems/binary-tree-paths/
///
struct Solution;
use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = vec![];
        let mut curr = vec![];
        Self::tree_path(root, &mut result, &mut curr);
        result
    }
    fn tree_path(
        root: Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<String>,
        curr: &mut Vec<String>,
    ) {
        match root {
            None => {}
            Some(rc) => {
                let borrow = rc.borrow();
                curr.push(borrow.val.to_string());
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                if left.is_none() && right.is_none() {
                    result.push(curr.join("->").to_string());
                    return;
                }
                if left.is_some() {
                    Self::tree_path(left, result, curr);
                    // 回溯
                    curr.pop();
                }
                if right.is_some() {
                    Self::tree_path(right, result, curr);
                    curr.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let sequential = vec![-1, 1, 2, 3, i32::MIN, 5];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        let ans = Solution::binary_tree_paths(root);
        let expect = hashset! {"1->2->5","1->3"}
            .into_iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        assert_eq!(HashSet::from_iter(ans.into_iter()), expect);
    }

    #[test]
    fn t2() {
        let sequential = vec![-1, 1];
        let root = TreeNode::build_tree_by_sequential_storage(&sequential, true);
        let ans = Solution::binary_tree_paths(root);
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(ans, hashset! {"1".to_string()});
    }
}
