/// 70 爬楼梯
///
/// https://leetcode.cn/problems/climbing-stairs/
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        // 初始化数组，上1阶有1种方法，上2阶有2种方法,
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            // 一次只能上1个或者2个台阶，所以第i个台阶只能有dp[i-1] + dp[i-2]种方法
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }
}
