///
/// [174. 地下城游戏](https://leetcode.cn/problems/dungeon-game/)
///
struct Solution;
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        // 采用动态规划（Dynamic Programming）逆向思考的方式解决。
        // 定义 dp[i][j] 表示从位置 (i,j) 到终点所需最少初始健康点数
        let mut dp = vec![vec![0; n]; m];

        // 初始化右下角
        dp[m - 1][n - 1] = 1.max(1 - dungeon[m - 1][n - 1]);

        // 填充最后一行
        for j in (0..n - 1).rev() {
            dp[m - 1][j] = 1.max(dp[m - 1][j + 1] - dungeon[m - 1][j]);
        }

        // 填充最后一列
        for i in (0..m - 1).rev() {
            dp[i][n - 1] = 1.max(dp[i + 1][n - 1] - dungeon[i][n - 1]);
        }

        // 填充其余部分
        for i in (0..m - 1).rev() {
            for j in (0..n - 1).rev() {
                let min_health_from_next = dp[i + 1][j].min(dp[i][j + 1]);
                dp[i][j] = 1.max(min_health_from_next - dungeon[i][j]);
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5]
            ]),
            7
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![vec![2, 1], vec![1, -1]]),
            1
        );
    }
}
