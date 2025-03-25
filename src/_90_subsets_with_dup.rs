///
///  90 子集 II (排序+子集+去重)
///
///  组合([1,2]和[2,1])是同一种解
///
/// https://leetcode.cn/problems/subsets-ii/description/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut nums = nums;
        // 排序
        nums.sort();
        // 放入空集合
        result.push(vec![]);
        Self::backtracking(
            0,
            &nums,
            &mut result,
            &mut vec![],
            &mut vec![false; nums.len()],
        );
        result
    }
    fn backtracking(
        index: usize,
        nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        sub: &mut Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        for i in index..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                // 去重
                continue;
            }
            sub.push(nums[i]);
            result.push(sub.clone());
            used[i] = true;
            Self::backtracking(i + 1, nums, result, sub, used);
            sub.pop();
            used[i] = false;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let ans = Solution::subsets_with_dup(vec![1, 2, 2]);
        let expect = hashset![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ];
        assert_eq!(expect.len(), ans.len());
        assert_eq!(expect, HashSet::from_iter(ans.into_iter()));
    }
    #[test]
    fn t2() {
        let ans = Solution::subsets_with_dup(vec![1]);
        let expect = hashset![vec![], vec![1],];
        assert_eq!(expect.len(), ans.len());
        assert_eq!(expect, HashSet::from_iter(ans.into_iter()));
    }
}
