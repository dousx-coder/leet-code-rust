struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() || s.len() <= 1 {
            return s;
        }
        let str_len = s.len();
        let mut max_start = 0;
        let mut max_end = 0;
        let mut max_len = 1;
        let mut dp = vec![vec![false; str_len]; str_len];
        for r in 1..str_len {
            for l in 0..r {
                let sl = &s[l..l + 1];
                let sr = &s[r..r + 1];
                if sl == sr && (r - l <= 2 || dp[l + 1][r - 1]) {
                    dp[l][r] = true;
                    let new_len = r - l + 1;
                    if new_len > max_len {
                        max_len = new_len;
                        max_start = l;
                        max_end = r;
                    }
                }
            }
        }
        String::from(&s[max_start..(max_end + 1)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome(String::from("abcbb")), "bcb");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome(String::from("aaaa")), "aaaa");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_palindrome(String::from("ac")), "a");
    }
}
