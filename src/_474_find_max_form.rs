/// 474 一和零
///
/// https://leetcode.cn/problems/ones-and-zeroes/
struct Solution;
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for str in strs {
            //  统计字符上0和1的个数
            let (count_1, count_0) = str.chars().fold((0, 0), |(ones, zeros), c| match c {
                '0' => (ones, (zeros + 1)),
                '1' => ((ones + 1), zeros),
                _ => panic!("Invalid char"),
            });
            for i in (count_0..=m).rev() {
                for j in (count_1..=n).rev() {
                    // dp[i][j] 表示当前背包中包含i个0和j个1的最多字符串个数
                    dp[i][j] = dp[i][j].max(dp[i - count_0][j - count_1] + 1);
                }
            }
        }
        dp[m][n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn t1() {
        let vec = vec!["10", "0001", "111001", "1", "0"]
            .iter()
            .map(|x| x.to_string())
            .collect_vec();
        assert_eq!(Solution::find_max_form(vec, 5, 3), 4);
    }
    #[test]
    fn t2() {
        let vec = vec!["10", "0", "1"]
            .iter()
            .map(|x| x.to_string())
            .collect_vec();
        assert_eq!(Solution::find_max_form(vec, 1, 1), 2);
    }
}
