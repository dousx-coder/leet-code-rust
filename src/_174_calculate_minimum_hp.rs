///
/// [174. 地下城游戏](https://leetcode.cn/problems/dungeon-game/)
///
struct Solution;
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        // 动态规划
        let (m, n) = (dungeon.len(), dungeon[0].len());
        // dp(i,j)的含义，如果是到达(i,j)所需要的最少初始hp
        let mut dp = vec![vec![0; n]; m];
        // 记录到达(i,j)时，剩余的hp值
        let mut hp = vec![vec![0; n]; m];

        dp[0][0] = if dungeon[0][0] < 0 { -dungeon[0][0] } else { 0 } + 1;
        // 达到初始位置，消耗hp之后最少要剩下1，否则就挂了
        hp[0][0] = 1;

        //  如果房间大于0，则是增加hp
        for i in 1..m {
            let c = dungeon[i][0];
            if c >= 0 {
                hp[i][0] = hp[i - 1][0] + c;
                dp[i][0] = dp[i - 1][0];
            } else {
                let surplus = hp[i - 1][0] + c;
                if surplus >= 1 {
                    // 剩余的hp足够
                    dp[i][0] = dp[i - 1][0];
                    hp[i][0] = surplus;
                } else {
                    // -2 -5 10
                    // 3   8
                    //  1 + 5 + 1
                    // 1- ( -2 + -5)
                    dp[i][0] = dp[i - 1][0] + c.abs();
                    hp[i][0] = 1;
                }
            }
        }
        for j in 1..n {
            let c = dungeon[0][j];
            if c >= 0 {
                hp[0][j] = hp[0][j - 1] + c;
                dp[0][j] = dp[0][j - 1];
            } else {
                let surplus = hp[0][j - 1] + c;
                if surplus >= 1 {
                    // 剩余的hp足够
                    hp[0][j] = surplus;
                    dp[0][j] = dp[0][j - 1];
                } else {
                    dp[0][j] = dp[0][j - 1] + c.abs();
                    hp[0][j] = 1;
                }
            }
        }
        for i in 1..m {
            for j in 1..n {
                let c = dungeon[i][j];
                // 有两条路可以走
                // (i-1,j)
                // (i,j-1)

                let dp1 = 0;
                let hp1 = 0;

                if dp[i - 1][j] <= dp[i][j - 1] {
                    if c >= 0 {
                        // 剩余的hp足够
                        hp[i][j] = hp[i - 1][j] + c;
                        dp[i][j] = dp[i - 1][j];
                    } else {
                        let surplus = hp[i - 1][j] + c;
                        if surplus >= 1 {
                            // 剩余的hp足够
                            hp[i][j] = surplus;
                            dp[i][j] = hp[i - 1][j];
                        } else {
                            dp[i][j] = dp[i - 1][j] + surplus.abs() + 1;
                            hp[i][j] = 1;
                        }
                    }
                } else {
                    if c >= 0 {
                        // 剩余的hp足够
                        hp[i][j] = hp[i][j - 1] + c;
                        dp[i][j] = dp[i][j - 1];
                    } else {
                        let surplus = hp[i][j - 1] + c;
                        if surplus >= 1 {
                            // 剩余的hp足够
                            hp[i][j] = surplus;
                            dp[i][j] = dp[i][j - 1];
                        } else {
                            // 这里应该取hp?
                            dp[i][j] = dp[i][j - 1] + surplus.abs() + 1;
                            hp[i][j] = 1;
                        }
                    }
                }
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
