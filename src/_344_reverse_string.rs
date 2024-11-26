///
///https://leetcode.cn/problems/reverse-string/description/
struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() || s.len() <= 1 {
            return;
        }
        let (mut i, mut j) = (0, s.len() - 1);
        while i <= j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
