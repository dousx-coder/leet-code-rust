/// LCR 083 全排列
///
///  https://leetcode.cn/problems/VvJkup/description/
///
struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::backtracking(&mut ans, &mut vec![], &nums, &mut vec![false; nums.len()]);
        ans
    }
    fn backtracking(
        ans: &mut Vec<Vec<i32>>,
        curr: &mut Vec<i32>,
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        if curr.len() == nums.len() {
            ans.push(curr.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            curr.push(nums[i]);
            Self::backtracking(ans, curr, nums, used);
            curr.pop();
            used[i] = false;
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
        let ans = Solution::permute(vec![1, 2, 3]);
        let expected = hashset! { vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],};
        assert_eq!(ans.len(), expected.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }

    #[test]
    fn t2() {
        let ans = Solution::permute(vec![0, 1]);
        let expected = hashset! {vec![0,1], vec![1, 0],};
        assert_eq!(ans.len(), expected.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }

    #[test]
    fn t3() {
        let ans = Solution::permute(vec![1]);
        let expected = vec![vec![1]];
        assert_eq!(ans, expected);
    }
}
