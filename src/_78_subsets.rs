/// 78 子集
///
/// https://leetcode.cn/problems/subsets/description/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        result.push(vec![]);
        Self::backtracking(0, &nums, &mut result, &mut vec![]);
        result
    }
    fn backtracking(
        pos: usize,
        nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        subset: &mut Vec<i32>,
    ) {
        let len = nums.len();
        if pos >= len {
            return;
        }
        for i in pos..len {
            subset.push(nums[i]);
            result.push(subset.clone());
            Self::backtracking(i + 1, nums, result, subset);
            subset.pop();
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
        let ans = Solution::subsets(vec![1, 2, 3]);
        let expect = hashset! {vec![],
        vec![1],
        vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]};
        assert_eq!(ans.len(), expect.len());
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(ans, expect)
    }
}
