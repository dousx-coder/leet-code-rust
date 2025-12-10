use std::collections::HashMap;
///
/// [97. 交错字符串](https://leetcode.cn/problems/interleaving-string/)
///
struct Solution;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1 = s1.chars().collect::<Vec<_>>();
        let s2 = s2.chars().collect::<Vec<_>>();
        let s3 = s3.chars().collect::<Vec<char>>();

        let mut counter = HashMap::new();
        for c in s1.iter() {
            *counter.entry(c).or_insert(0) += 1;
        }
        for c in s2.iter() {
            *counter.entry(c).or_insert(0) += 1;
        }

        for ele in s3.iter() {
            if let Some(v) = counter.get_mut(ele) {
                *v -= 1;
                if *v < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        Self::dfs(
            &s1,
            &s2,
            &s3,
            0,
            0,
            &mut vec![vec![-1; s2.len() + 1]; s1.len() + 1],
        )
    }

    fn dfs(
        s1: &Vec<char>,
        s2: &Vec<char>,
        s3: &Vec<char>,
        i: usize,
        j: usize,
        cache: &mut Vec<Vec<i8>>,
    ) -> bool {
        if i == s1.len() && j == s2.len() {
            return true;
        }
        if i + j >= s3.len() {
            return false;
        }
        if cache[i][j] != -1 {
            return cache[i][j] == 1;
        }
        if i < s1.len() && j < s2.len() && s1[i] == s3[i + j] && s2[j] == s3[i + j] {
            return Self::dfs(s1, s2, s3, i + 1, j, cache)
                || Self::dfs(s1, s2, s3, i, j + 1, cache);
        }
        let mut ans = false;

        if i < s1.len() && s1[i] == s3[i + j] {
            ans = ans || Self::dfs(s1, s2, s3, i + 1, j, cache);
        }
        if j < s2.len() && s2[j] == s3[i + j] {
            ans = ans || Self::dfs(s1, s2, s3, i, j + 1, cache);
        }
        cache[i][j] = if ans { 1 } else { 0 };
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbcbcac".to_string()
            ),
            true
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::is_interleave("".to_string(), "abc".to_string(), "abc".to_string()),
            true
        );
    }
}
