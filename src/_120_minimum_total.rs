///
/// [120. 三角形最小路劲的和](https://leetcode.cn/problems/triangle/)
///
struct Solution;
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![];
        dp.push(vec![triangle[0][0]]);
        for i in 1..triangle.len() {
            let len = triangle[i].len();
            let mut curr_dp = vec![0; len];
            for j in 0..len {
                let t1 = if j == len - 1 {
                    i32::MAX
                } else {
                    dp[dp.len() - 1][j]
                };
                let t2 = if j == 0 {
                    i32::MAX
                } else {
                    dp[dp.len() - 1][j - 1]
                };
                curr_dp[j] = t1.min(t2) + triangle[i][j];
            }
            dp.push(curr_dp);
        }
        // 取最后一行的最小值
        *dp.last().unwrap().iter().min().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
