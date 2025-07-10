use std::collections::HashSet;

///
/// [47. 全排列Ⅱ](https://leetcode.cn/problems/permutations-ii/)
///
struct Solution;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: HashSet<Vec<i32>> = HashSet::new();
        let mut nc = nums.clone();
        Self::permute_unique_recursion(&mut result, &mut nc, 0, nums.len() - 1);
        result.into_iter().collect()
    }
    fn permute_unique_recursion(
        result: &mut HashSet<Vec<i32>>,
        nums: &mut Vec<i32>,
        start: usize,
        end: usize,
    ) {
        if start == end {
            result.insert(nums.clone());
            return;
        }
        for i in start..=end {
            nums.swap(start, i);
            Self::permute_unique_recursion(result, nums, start + 1, end);
            nums.swap(start, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    #[test]
    fn t1() {
        let ans = Solution::permute_unique(vec![1, 1, 2]);
        let set = HashSet::from_iter(ans.into_iter());
        assert_eq!(set, hashset![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]])
    }
}
