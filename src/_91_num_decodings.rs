///
/// [91.解码方法](https://leetcode.cn/problems/decode-ways/)
///
struct Solution;
impl Solution {
    /// 爬楼梯复杂版本
    ///
    /// 如果连续的两位数符合条件，就相当于一个上楼梯的题目，可以有两种选法：
    ///
    ///     1.一位数决定一个字母
    ///
    ///     2.两位数决定一个字母
    ///
    /// 就相当于dp(i) = dp[i-1] + dp[i-2]
    ///
    /// 不符合条件，又有两种情况
    ///
    ///     1.当前数字是0： dp[i] = dp[i-2]
    ///
    ///     2.当前数字不是0：dp[i] = dp[i-1]
    ///
    pub fn num_decodings(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        if chs[0] == '0' {
            return 0;
        }
        let len = chs.len();

        let mut dp = vec![0; len + 1];
        dp[0] = 1;

        for i in 0..len {
            dp[i + 1] = if chs[i] == '0' { 0 } else { dp[i] };

            if i > 0 && (chs[i - 1] == '1' || (chs[i - 1] == '2' && chs[i] <= '6')) {
                dp[i + 1] += dp[i - 1];
            }
        }

        dp[len]
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::num_decodings("11106".to_string()), 2);
    }
}
