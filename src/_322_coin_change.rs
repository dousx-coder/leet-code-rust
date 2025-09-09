///
/// [322. 零钱兑换](https://leetcode.cn/problems/coin-change/)
///
///
struct Solution;
impl Solution {
    ///给你一个整数数组 `coins` ，表示不同面额的硬币；以及一个整数 `amount` ，表示总金额。
    ///
    /// 计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 `-1` 。
    ///
    /// 你可以认为每种硬币的数量是无限的。
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            // 如果总金额为0，直接返回0
            // 因为不需要任何硬币来凑成0
            return 0;
        }
        if amount < 0 {
            // 如果总金额小于0，直接返回-1
            return -1;
        }
        let amount = amount as usize;

        // 金币一定是正整数，因此可以直接转换为 usize
        let coins: Vec<usize> = coins.iter().map(|&e| e as usize).collect();

        // 创建一个数组 dp，用于存储每个金额所需的最小硬币数
        let mut dp = vec![i32::MAX; (amount + 1) as usize];

        // 0元需要0个硬币
        dp[0] = 0;

        // 遍历每个硬币
        for &coin in &coins {
            // 对于每个硬币，更新从 coin 到 amount 的所有金额
            for j in coin..=amount {
                let k = j - coin;
                if dp[k] != i32::MAX {
                    // 如果 j-coin 的金额可以被凑成
                    dp[j] = dp[j].min(dp[k] + 1);
                }
            }
        }

        if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        }
        
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
