use itertools::Itertools;

/// 416 分割等和子集
///
/// https://leetcode.cn/problems/partition-equal-subset-sum/
struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let target = (sum / 2) as usize;
        let nums = nums.into_iter().map(|x| x as usize).collect_vec();
        // 思路：看成01背包问题
        // 这些数字放入一个背包，背包容量为target，
        // 背包装满，则说明存在一种方案使得背包装满，且装满后背包中物品的总和为target。
        // 数字5看成重量为5，价值为5，即单位重量/价值=1
        let mut dp = vec![0; target + 1];
        for i in 0..nums.len() {
            let num = nums[i];
            for j in (num..=target).rev() {
                dp[j] = dp[j].max(dp[j - num] + num);
            }
        }
        dp[target] == target
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    }
}
