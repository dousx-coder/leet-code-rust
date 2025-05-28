use itertools::Itertools;

/// 494 目标和
///
/// https://leetcode.cn/problems/target-sum/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        Self::backtracking(0, &nums, target)
    }
    fn backtracking(pos: usize, nums: &Vec<i32>, target: i32) -> i32 {
        if pos == nums.len() {
            return if target == 0 { 1 } else { 0 };
        }
        let mut ans = 0;
        ans += Self::backtracking(pos + 1, nums, target - nums[pos]);
        ans += Self::backtracking(pos + 1, nums, target + nums[pos]);
        ans
    }

    ///
    /// 动态规划解法
    ///
    /// 假设数字前面添加+号的数字总和为left，添加-号的总和为right，则可得到
    ///
    /// 等式1:left+right=sum(nums)
    ///
    /// 等式2:left-right=target
    ///
    /// 由等式1和等式2可得：2left=sum(nums)+target
    ///
    /// left=(sum(nums)+target)/2,即转换为背包问题，
    /// 求nums中任取多少个数字达到left共有多少种方式
    fn dp(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        // 用sum+target，leetcode会过不去?
        if sum < target || (sum - target) % 2 != 0 {
            return 0;
        }
        let right = ((sum - target) / 2) as usize;
        let mut dp = vec![0; right + 1];
        dp[0] = 1;
        for x in nums {
            for j in (x as usize..=right).rev() {
                dp[j] += dp[j - x as usize];
            }
        }
        dp[right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3);
        assert_eq!(5, ans);
    }

    #[test]
    fn t2() {
        let ans = Solution::find_target_sum_ways(vec![1], 1);
        assert_eq!(1, ans);
    }

    #[test]
    fn t3() {
        let ans = Solution::find_target_sum_ways(vec![1], 0);
        assert_eq!(0, ans);
    }

    #[test]
    fn t4() {
        let ans = Solution::find_target_sum_ways(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            1,
        );
        assert_eq!(524288, ans);
    }

    #[test]
    fn t5() {
        let ans = Solution::dp(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            1,
        );
        assert_eq!(524288, ans);
    }
    #[test]
    fn t6() {
        let ans = Solution::dp(vec![100], -200);
        assert_eq!(0, ans);
    }
    #[test]
    fn t7() {
        let ans = Solution::dp(vec![1000], -1000);
        assert_eq!(1, ans);
    }
}
