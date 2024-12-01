use std::ops::Add;

///
/// `459. 重复的子字符串`
///
/// https://leetcode.cn/problems/repeated-substring-pattern/
///
///
struct Solution;
impl Solution {
    fn violence_solution(s: String) -> bool {
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
    fn splice_solution(s: String) -> bool {
        let len = s.len();
        if len == 0 {
            return false;
        }
        let x = &s;
        //  如果s是由n个重复字串p，那么s=n*p (n>=2)
        //  2s=2n*p >= 4p
        // 去掉首位字符，然后查找s，如果包含，则说明s是由重复子串组成
        let mut s = String::from(x).add(x);
        s.remove(0);
        s.remove(s.len() - 1);
        s.find(x).is_some()
    }

    pub fn repeated_substring_pattern(s: String) -> bool {
        Solution::splice_solution(s)
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
