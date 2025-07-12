///
/// [263. 丢失的数字](https://leetcode.cn/problems/missing-number/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let n = nums.len();
        for i in (1..=n).rev() {
            // 从n开始，一边减,一边加，防止溢出
            sum += nums[i - 1] - (i as i32);
        }
        sum.abs()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
