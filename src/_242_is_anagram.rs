///
/// https://leetcode.cn/problems/valid-anagram/
struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut vec = vec![0; 26];
        for (index, char) in s.as_bytes().iter().enumerate() {
            let i = char - 97;
            vec[i as usize] += 1;
        }
        for (index, char) in t.as_bytes().iter().enumerate() {
            let i = char - 97;
            vec[i as usize] -= 1;
        }
        vec.iter().all(|&x| x == 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::is_anagram("abcde".to_string(), "bcdea".to_string());
        assert_eq!(ans, true);
    }

    #[test]
    fn t2() {
        let ans = Solution::is_anagram("abc".to_string(), "adc".to_string());
        assert_eq!(ans, false);
    }
}
