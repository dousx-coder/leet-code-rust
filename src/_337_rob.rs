use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

///
/// [337. 打家劫舍Ⅲ](https://leetcode.cn/problems/house-robber-iii/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cache: HashMap<*const RefCell<TreeNode>, i32> = HashMap::new();
        Self::post_dfs(root, &mut cache)
    }
    /// 后续遍历
    fn post_dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        cache: &mut HashMap<*const RefCell<TreeNode>, i32>,
    ) -> i32 {
        match root {
            None => 0,
            Some(rc) => {
                let borrow = rc.borrow();
                let left = borrow.left.clone();
                let right = borrow.right.clone();
                let val = borrow.val;
                if left.is_none() && right.is_none() {
                    return val;
                }
                if let Some(value) = cache.get(&Rc::as_ptr(&rc)) {
                    return *value;
                }
                // 偷父节点
                let mut sum1 = val;
                match left.clone() {
                    None => {}
                    Some(rc) => {
                        let x = rc.borrow();
                        sum1 += Self::post_dfs(x.left.clone(), cache)
                            + Self::post_dfs(x.right.clone(), cache);
                    }
                }
                match right.clone() {
                    None => {}
                    Some(rc) => {
                        let x = rc.borrow();
                        sum1 += Self::post_dfs(x.left.clone(), cache)
                            + Self::post_dfs(x.right.clone(), cache);
                    }
                }
                // 不偷父节点
                let mut sum2 = Self::post_dfs(left, cache) + Self::post_dfs(right, cache);
                let ans = max(sum1, sum2);
                cache.insert(Rc::as_ptr(&rc), ans);
                ans
            }
        }
    }

    /// 层序 AC 50%  
    /// 层序思路不对 不一定非要间隔一个 4 1 2 3 可以只偷 4和 3 最大值是7 而非4+2
    fn level(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level_order_traversal = vec![];
        // 下一层
        let mut next_level_node = VecDeque::new();
        // 当前层
        let mut curr_level_node = VecDeque::new();
        curr_level_node.push_back(root.clone());
        let mut curr_level_traversal = vec![];
        loop {
            match curr_level_node.pop_front() {
                Some(node) => match node {
                    Some(rc) => {
                        curr_level_traversal.push(rc.borrow().val);
                        next_level_node.push_back(rc.borrow().left.clone());
                        next_level_node.push_back(rc.borrow().right.clone());
                    }
                    None => {}
                },
                None => {}
            }
            if curr_level_node.is_empty() && !next_level_node.is_empty() {
                while let Some(rc) = next_level_node.pop_front() {
                    curr_level_node.push_back(rc);
                }
                if !curr_level_traversal.is_empty() {
                    level_order_traversal.push(curr_level_traversal);
                    curr_level_traversal = vec![];
                }
                continue;
            }
            if curr_level_node.is_empty() && next_level_node.is_empty() {
                if !curr_level_traversal.is_empty() {
                    level_order_traversal.push(curr_level_traversal);
                }
                break;
            }
        }
        let len = level_order_traversal.len();
        let mut sum1 = 0;
        let mut sum2 = 0;
        for i in 0..len {
            if i % 2 == 0 {
                sum1 += level_order_traversal[i].iter().sum::<i32>();
            } else {
                sum2 += level_order_traversal[i].iter().sum::<i32>();
            }
        }
        max(sum1, sum2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let m = i32::MIN;
        let sequential = vec![vec![3], vec![2, 3], vec![m, 3, m, 1]];
        let root = TreeNode::build_tree_by_two_sequential(sequential, true);
        let sum = Solution::level(root);
        assert_eq!(sum, 7);
    }

    #[test]
    fn t2() {
        let m = i32::MIN;
        let sequential = vec![vec![3], vec![4, 5], vec![1, 3, m, 1]];
        let root = TreeNode::build_tree_by_two_sequential(sequential, true);
        let sum = Solution::level(root);
        assert_eq!(sum, 9);
    }

    #[test]
    fn t3() {
        let m = i32::MIN;
        let sequential = vec![
            vec![4],
            vec![1, m],
            vec![2, m, m, m],
            vec![3, m, m, m, m, m, m, m],
        ];
        let root = TreeNode::build_tree_by_two_sequential(sequential, true);
        let sum = Solution::rob(root);
        assert_eq!(sum, 7);
    }
}
