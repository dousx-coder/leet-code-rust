/// 509 Fibonacci Number
///
/// https://leetcode.cn/problems/fibonacci-number/
struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0, 1];
        for i in 2..=n {
            dp.push(dp[i - 1] + dp[i - 2]);
        }
        dp[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::fib(2), 1);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::fib(3), 2);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::fib(4), 3);
    }
}
