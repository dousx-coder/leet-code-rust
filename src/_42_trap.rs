use std::cmp::{max, min};
///
/// https://leetcode.cn/problems/trapping-rain-water/description/
struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len == 0 {
            return 0;
        }
        let mut left_max = vec![0; len];
        let mut right_max = vec![0; len];
        for i in 1..len {
            left_max[i] = max(left_max[i - 1], height[i - 1]);
            right_max[len - i - 1] = max(right_max[len - i], height[len - i]);
        }
        let mut result = 0;
        for i in 0..len {
            result += max(0, min(left_max[i], right_max[i]) - height[i]);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
