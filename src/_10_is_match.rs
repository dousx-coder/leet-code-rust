///
/// 10 正则表达式匹配
///
/// https://leetcode.cn/problems/regular-expression-matching/
///
struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let m = s.len();
        let p = p.chars().collect::<Vec<char>>();
        let n = p.len();
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for i in 0..=m {
            for j in 1..=n {
                let x = p[j - 1];
                if p[j - 1] == '*' {
                    dp[i][j] = dp[i][j - 2];
                    if Self::matches(&s, &p, i, j - 1) {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                } else {
                    if Self::matches(&s, &p, i, j) {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }
        dp[m][n]
    }
    fn matches(s: &Vec<char>, p: &Vec<char>, i: usize, j: usize) -> bool {
        if i == 0 {
            return false;
        }
        if p[j - 1] == '.' {
            return true;
        }
        s[i - 1] == p[j - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn t1() {
        let s = String::from("aa");
        let p = String::from("a");
        assert_eq!(Solution::is_match(s, p), false);
    }

    #[test]
    fn t2() {
        let s = String::from("aa");
        let p = String::from("a*");
        assert_eq!(Solution::is_match(s, p), true);
    }

    #[test]
    fn t3() {
        let s = String::from("ab");
        let p = String::from(".*");
        assert_eq!(Solution::is_match(s, p), true);
    }
}
