///
/// [78. 子集](https://leetcode.cn/problems/subsets/description/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtracking(0, &nums, &mut result, &mut vec![]);
        result
    }
    fn backtracking(
        pos: usize,
        nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        subset: &mut Vec<i32>,
    ) {
        if pos == nums.len() {
            result.push(subset.clone());
            return;
        }
        // 不包含 nums[pos] 的情况
        Self::backtracking(pos + 1, nums, result, subset);
        // 包含 nums[pos] 的情况
        subset.push(nums[pos]);
        Self::backtracking(pos + 1, nums, result, subset);
        subset.pop();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let ans = Solution::subsets(vec![1, 2, 3]);
        let expect = hashset! {vec![],
        vec![1],
        vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]};
        assert_eq!(ans.len(), expect.len());
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(ans, expect)
    }
}
