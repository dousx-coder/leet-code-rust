use std::collections::HashSet;
///
/// [491. 非递减子序列](https://leetcode.cn/problems/non-decreasing-subsequences/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // nums是无序的，只能用hashSet+used标记双重去重(used剪枝)
        let mut ans = Vec::new();
        let used = &mut vec![false; nums.len()];
        let path = &mut vec![];
        Self::backtracking1(0, &nums, path, &mut ans);
        ans
    }
    /// set标记当前层已用过的元素，避免重复
    ///
    /// 耗时：0ms，内存：4MB
    fn backtracking1(start: usize, nums: &Vec<i32>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if start >= nums.len() {
            return;
        }
        // 同一个父节点下，当前层不能出现相同的元素
        let mut used = HashSet::new();
        for i in start..nums.len() {
            if used.contains(&nums[i]) {
                // 使用set标记当前层已用过的元素
                continue;
            }
            if !path.is_empty() && nums[i] < *path.last().unwrap() {
                continue;
            }

            used.insert(nums[i]);
            path.push(nums[i]);
            if path.len() >= 2 {
                ans.push(path.clone());
            }
            Self::backtracking1(i + 1, nums, path, ans);
            path.pop();
        }
    }
    /// 数组去重，根据数值范围创建数组长度为201的bool数组，避免使用set
    /// 耗时：1ms，内存：4MB
    fn backtracking2(start: usize, nums: &Vec<i32>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if start >= nums.len() {
            return;
        }
        // 数值范围[-100, 100]
        // 同一个父节点下，当前层不能出现相同的元素
        let mut used = vec![false; 201];
        for i in start..nums.len() {
            let j = (nums[i] + 100) as usize;
            if used[j] {
                continue;
            }
            if !path.is_empty() && nums[i] < *path.last().unwrap() {
                continue;
            }
            used[j] = true;
            path.push(nums[i]);
            if path.len() >= 2 {
                ans.push(path.clone());
            }
            Self::backtracking2(i + 1, nums, path, ans);
            path.pop();
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

    #[test]
    fn t3() {
        let expected = hashset! {vec![4,6],vec![4,6,7],vec![4,6,7,7],vec![4,7],vec![4,7,7],vec![6,7],vec![6,7,7],vec![7,7]};
        let nums = vec![4, 6, 7, 7];
        let mut ans = Vec::new();
        let used = &mut vec![false; nums.len()];
        let path = &mut vec![];
        Solution::backtracking2(0, &nums, path, &mut ans);
        assert_eq!(expected.len(), ans.len());
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(expected, ans);
    }
}
