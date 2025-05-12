/// 746 最小花费爬楼梯
///
/// https://leetcode.cn/problems/min-cost-climbing-stairs/
struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // dp[i] 表示到达第i个台阶的最小花费
        let len = cost.len();

        let mut dp = vec![0; 2];
        // 可以选择从下标为 0 或下标为 1 的台阶开始爬楼梯，即dp[0]和dp[1] 都可以设置为0;
        // dp[1]设置为0，则表示从下标1开始爬
        for i in 2..=len {
            // 一次可以跳跃1个台阶或者2个台阶，所以下面两种情况取最小值
            // 从i-1台阶跳过来
            let a = cost[i - 1] + dp[i - 1];
            // 从i-2台阶跳过来
            let b = cost[i - 2] + dp[i - 2];
            dp.push(a.min(b));
        }
        dp[len]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
