use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
///  [230. 二叉搜索树中第K小的元素](https://leetcode.cn/problems/kth-smallest-element-in-a-bst/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // 中序 迭代法
        let mut stack = Vec::new();
        let mut curr = root;
        let mut count = 0;
        while curr.is_some() || !stack.is_empty() {
            // 尽可能往左走，并把路径上的节点压入栈中
            while let Some(curr_rc) = curr {
                stack.push(curr_rc.clone());
                curr = curr_rc.borrow().left.clone();
            }
            // 弹出栈顶节点，访问它并转向右子树
            if let Some(stack_pop) = stack.pop() {
                count += 1;
                if count == k {
                    return stack_pop.borrow().val;
                }
                curr = stack_pop.borrow().right.clone();
            }
        }
        panic!("kth_smallest failed");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let tree = TreeNode::build_bst(&nums);
        let ans = Solution::kth_smallest(tree, 2);
        assert_eq!(ans, -3);
    }

    #[test]
    fn t2() {
        let preorder = vec![3, 1, 2, 4];
        let inorder = vec![1, 2, 3, 4];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::kth_smallest(root, 1);
        assert_eq!(ans, 1);
    }
}
