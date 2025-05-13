/// 63 不同路径II
/// 
/// https://leetcode.cn/problems/unique-paths-ii/description/
struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        for row in 0..m {
            if obstacle_grid[row][0] == 1 {
                // 遇到路径，则后面的都不能到达(题意只能向右/向下)
                break;
            }
            dp[row][0] = 1;
        }
        for col in 0..n {
            if obstacle_grid[0][col] == 1 {
                break;
            }
            dp[0][col] = 1;
        }
        for row in 1..m {
            for col in 1..n {
                if obstacle_grid[row][col] == 1 {
                    continue;
                }
                dp[row][col] = dp[row - 1][col] + dp[row][col - 1];
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
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
    }
}
