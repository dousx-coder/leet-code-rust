///
/// [53. 最大子数组和](https://leetcode.cn/problems/maximum-subarray/)
///
struct Solution;
impl Solution {
    /// 找出一个具有最大和的连续子数组
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr = nums[0];
        let mut max = nums[0];
        for &num in nums.iter().skip(1) {
            curr = num.max(curr + num);
            max = max.max(curr);
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
