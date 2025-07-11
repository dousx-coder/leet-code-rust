use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
///[968. 监控摄像头](https://leetcode.cn/problems/binary-tree-cameras/)
///
struct Solution;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        if Solution::postorder_traversal(&mut ans, root) == 0 {
            ans += 1;
        }
        ans
    }
    fn postorder_traversal(ans: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => {
                //  当前节点是空，认为有覆盖
                2
            }
            Some(node) => {
                // 后序遍历
                let mut bm = node.borrow_mut();
                let left = Solution::postorder_traversal(ans, bm.left.clone());
                let right = Solution::postorder_traversal(ans, bm.right.clone());
                if left == 2 && right == 2 {
                    // 左右孩子均被覆盖，当前节点无需加装摄像头
                    return 0;
                }
                if left == 0 || right == 0 {
                    // 左右节点存在无覆盖的，则当前节点需要加装摄像头
                    *ans += 1;
                    return 1;
                }
                // 左右节点有覆盖且至少一个装了摄像头，则当前节点无需安装摄像头
                2
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    /// build_binary_tree 要求每个节点数值都不一样，但是体重给的节点值都是0，所以这里先用其他值构建二叉树，然后再替换成0
    fn build_test_data(preorder: &Vec<i32>, inorder: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = TreeNode::build_binary_tree(&preorder, &inorder);
        replace_zero(&mut root);
        root
    }

    fn replace_zero(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            node.val = 0;
            replace_zero(&mut node.left);
            replace_zero(&mut node.right);
        }
    }
    #[test]
    fn t1() {
        let preorder = vec![1, 2, 3, 4];
        let inorder = vec![3, 2, 4, 1];
        let root = build_test_data(&preorder, &inorder);
        let ans = Solution::min_camera_cover(root);
        assert_eq!(ans, 1);
    }
    #[test]
    fn t2() {
        let preorder = vec![1, 2, 3, 4, 5];
        let inorder = vec![4, 5, 3, 2, 1];
        let root = build_test_data(&preorder, &inorder);
        let ans = Solution::min_camera_cover(root);
        assert_eq!(ans, 2);
    }
}
