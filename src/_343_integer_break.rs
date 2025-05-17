/// 343 整数拆分
///
///
/// https://leetcode.cn/problems/integer-break/
struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        // 拆分一个数n 使之乘积最大，那么一定是拆分成m个近似相同的子数相乘才是最大的。(类似k小于0的二次函数)
        let n = n as usize;
        // dp[i]：分拆数字i，可以得到的最大乘积为dp[i]
        let mut dp = vec![0; n + 1];
        dp[1] = 0;
        dp[2] = 1;
        dp[3] = 2;
        for i in 4..=n {
            for j in 1..=i / 2 {
                // 有2种渠道得到dp[i]
                // 1: j * (i - j) 直接相乘。
                // 2: j * dp[i - j]，相当于是拆分(i - j)
                dp[i] = dp[i].max(j * dp[i - j]).max(j * (i - j));
            }
        }
        dp[n] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::integer_break(2), 1);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::integer_break(3), 2);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::integer_break(10), 36);
    }
}
