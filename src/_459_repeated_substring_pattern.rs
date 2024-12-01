///
/// `459. 重复的子字符串`
///
/// https://leetcode.cn/problems/repeated-substring-pattern/
///
///
struct Solution;
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();
        if len == 0 {
            return false;
        }
        for i in 1..len {
            let sub1 = &s[..i];
            for j in (i..len).step_by(i) {
                let end = j + i;
                if end > len {
                    // 剩余的字符串长度不够
                    break;
                }
                let sub2 = &s[j..end];
                if sub1 != sub2 {
                    // 字符串不相等
                    break;
                }
                if end == len {
                    return true;
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::repeated_substring_pattern("abc".to_string()),
            false
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::repeated_substring_pattern("abab".to_string()),
            true
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::repeated_substring_pattern("abcabcabcabc".to_string()),
            true
        );
    }
}
