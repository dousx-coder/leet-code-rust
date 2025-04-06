/// 561 数组拆分
///
/// https://leetcode.cn/problems/array-partition/description/
struct Solution;
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        // 排序 取偶数下标上的数字和
        nums.sort_unstable();
        let len = nums.len();
        let sum = (0..len).step_by(2).map(|i| nums[i]).sum::<i32>();
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
