///
/// [58. 最后一个单词的长度](https://leetcode.cn/problems/length-of-last-word/)
///
struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(' ').last().map(|s| s.len()).unwrap_or(0) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
