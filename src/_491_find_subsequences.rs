use std::collections::HashSet;

/// 491 非递减子序列
///
/// https://leetcode.cn/problems/non-decreasing-subsequences/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // nums是无序的，只能用hashSet+used标记双重去重(used剪枝)
        let mut ans = HashSet::new();
        let used = &mut vec![false; nums.len()];
        let path = &mut vec![];
        Self::backtracking(0, &nums, used, path, &mut ans);
        ans.into_iter().collect()
    }
    fn backtracking(
        start: usize,
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        path: &mut Vec<i32>,
        ans: &mut HashSet<Vec<i32>>,
    ) {
        if start >= nums.len() {
            return;
        }
        for i in start..nums.len() {
            if used[i] {
                continue;
            }
            if path.is_empty() || nums[i] >= *path.last().unwrap() {
                used[i] = true;
                path.push(nums[i]);
                if path.len() >= 2 {
                    ans.insert(path.clone());
                }
                Self::backtracking(i + 1, nums, used, path, ans);
                path.pop();
                used[i] = false;
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
        let expected = hashset! {vec![4,6],vec![4,6,7],vec![4,6,7,7],vec![4,7],vec![4,7,7],vec![6,7],vec![6,7,7],vec![7,7]};
        let ans = Solution::find_subsequences(vec![4, 6, 7, 7]);
        assert_eq!(expected.len(), ans.len());
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(expected, ans);
    }

    #[test]
    fn t2() {
        let expected = hashset! {vec![4,4]};
        let ans = Solution::find_subsequences(vec![4, 4, 3, 2, 1]);
        assert_eq!(expected.len(), ans.len());
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(expected, ans);
    }
}
