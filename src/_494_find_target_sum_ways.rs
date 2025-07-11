use itertools::Itertools;
///
///[494. 目标和](https://leetcode.cn/problems/target-sum/?envType=problem-list-v2&envId=backtracking)
///
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
        // 针对特殊情况的分析：如 nums=[1000], target=-1000
        // 此时只能将唯一数字取负，有一种合法方式。
        // 若通过 left=(sum + target)/2 计算，则会得到left=0，但nums中没有0导致错误结果
        if sum < target || (sum - target) % 2 != 0 {
            return 0;
        }
        let right = ((sum - target) / 2) as usize;
        let mut dp = vec![0; right + 1];
        dp[0] = 1;

        // dp[j] 表示从前缀数组中选出若干个元素组成和为j的方式数量
        // 例如：dp[5]表示 数组中取任意数字，共有多少种组合方式使得总和为5
        //
        // 如果数组中存在数字1，则dp[5]=dp[4]，如果存在数组1，则组成dp[4]的这些组合中再加上1，就能得到组合和为5
        // 所以dp[right]应该是下面所有情况总和
        //
        // 如果存在right，则加上dp[0]
        // 如果存在right-1，则加上dp[1]
        // 如果存在right-2，则加上dp[2]
        // ...

        for x in nums {
            let val = x as usize;
            for j in (val..=right).rev() {
                // 如果此时val是7
                // dp[10]=dp[10]+dp[3]
                dp[j] += dp[j - val];
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
