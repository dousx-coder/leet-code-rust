struct Solution;
///
///
/// https://leetcode.cn/problems/valid-parentheses/description/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec = Vec::new();
        let a = ('(', ')');
        let b = ('{', '}');
        let c = ('[', ']');
        for x in s.chars() {
            if x == a.0 || x == b.0 || x == c.0 {
                vec.push(x);
            } else {
                if vec.len() == 0 {
                    return false;
                }
                let last = vec[vec.len() - 1];
                if x == a.1 && last == a.0 {
                    vec.pop();
                    continue;
                }
                if x == b.1 && last == b.0 {
                    vec.pop();
                    continue;
                }
                if x == c.1 && last == c.0 {
                    vec.pop();
                    continue;
                }
                return false;
            }
        }
        vec.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::is_valid("([])".to_string()), true);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::is_valid("[)".to_string()), false);
    }
}