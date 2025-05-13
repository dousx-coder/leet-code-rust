/// 62 不同路径
///
/// https://leetcode.cn/problems/unique-paths/
struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];
        // 左边第一列都是只有1种走法
        for row in 0..m {
            dp[row][0] = 1
        }
        // 上边第一行都是只有1种走法
        for col in 0..n {
            dp[0][col] = 1
        }
        for i in 1..m {
            for j in 1..n {
                // dp[i][j] 表示走到 (i, j) 的路径数，只能从左边和上边走到(i,j)处
                // 所以是dp[i - 1][j] + dp[i][j - 1]两种情况相加
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m - 1][n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
