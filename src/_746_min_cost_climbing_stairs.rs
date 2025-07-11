///
/// [746. 最小花费爬楼梯](https://leetcode.cn/problems/min-cost-climbing-stairs/)
///
struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // dp[i] 表示到达第i个台阶的最小花费
        let len = cost.len();

        // 初始化动态规划数组，dp[0]和dp[1]都为0，表示起始点无需花费
        let mut dp = vec![0, 0];

        // 从第2个台阶开始计算，直到终点
        for i in 2..=len {
            // 计算从i-1台阶跳到当前台阶的总花费
            let a = cost[i - 1] + dp[i - 1];
            // 计算从i-2台阶跳到当前台阶的总花费
            let b = cost[i - 2] + dp[i - 2];
            // 保存到达当前台阶的最小花费
            dp.push(a.min(b));
        }

        // 返回到达终点的最小花费
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
