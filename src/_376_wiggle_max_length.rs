///
///  379 摆动序列
///
/// https://leetcode.cn/problems/wiggle-subsequence/
struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 2 {
            return len as i32;
        }
        let mut pre_diff = 0;
        // 将nums[0] 当成摆动子序列的开头
        let mut count = 1;
        for i in 1..len {
            let mut cur_diff = nums[i] - nums[i - 1];
            if cur_diff > 0 && pre_diff <= 0 || cur_diff < 0 && pre_diff >= 0 {
                count += 1;
                pre_diff = cur_diff;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
    }
    #[test]
    fn t3() {
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2
        );
    }
    #[test]
    fn t4() {
        //    2 - 2 - 2 - 2
        //   /              \
        //  1                1
        assert_eq!(Solution::wiggle_max_length(vec![1, 2, 2, 2, 2, 1]), 3);
    }
    #[test]
    fn t5() {
        //                    4
        //                   /
        //                  3
        //                 /
        //    2 - 2 - 2 - 2
        //   /
        //  1
        assert_eq!(Solution::wiggle_max_length(vec![1, 2, 2, 2, 3, 4]), 2);
    }
}
