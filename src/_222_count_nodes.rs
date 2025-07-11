use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [222. 完全二叉树的节点个数](https://leetcode.cn/problems/count-complete-tree-nodes/)
///
struct Solution;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(rc) => {
                Self::count_nodes(rc.borrow().left.clone())
                    + Self::count_nodes(rc.borrow().right.clone())
                    + 1
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let sequential = vec![-1, 1, 2, 3, 4, 5, 6];
        let root = TreeNode::build_complete_tree(&sequential);
        let ans = Solution::count_nodes(root);
        assert_eq!(ans, 6);
    }
}
