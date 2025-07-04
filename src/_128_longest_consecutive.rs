/// 128 最长连续序列
///
/// https://leetcode.cn/problems/longest-consecutive-sequence/?envType=study-plan-v2&envId=top-100-liked
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut cnt = HashMap::new();

        for &num in &nums {
            if !cnt.contains_key(&num) {
                let left = *cnt.get(&(num - 1)).unwrap_or(&0);
                let right = *cnt.get(&(num + 1)).unwrap_or(&0);
                let sum = left + right + 1;
                cnt.insert(num, sum);
                ans = ans.max(sum);
                // Update the boundaries
                cnt.insert(num - left, sum);
                cnt.insert(num + right, sum);
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
