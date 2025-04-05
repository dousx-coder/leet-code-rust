///
/// 53 最大子数组和
///
/// https://leetcode.cn/problems/maximum-subarray/
struct Solution;
impl Solution {
    /// 找出一个具有最大和的连续子数组
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 当前连续子数组的和
        let mut curr = nums[0];
        // 最大连续子数组的和
        let mut max = nums[0];
        for i in 1..nums.len() {
            if curr < 0 {
                curr = nums[i];
            } else {
                curr += nums[i]
            }
            if curr > max {
                max = curr;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
