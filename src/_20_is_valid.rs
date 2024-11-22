struct Solution;
///
///
/// https://leetcode.cn/problems/valid-parentheses/description/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let pairs = vec![('(', ')'), ('{', '}'), ('[', ']')];
        let mut stack_vec = Vec::new();
        for x in s.chars() {
            let is_push = pairs.iter().any(|pair| {
                if pair.0 == x {
                    stack_vec.push(x);
                    return true;
                }
                false
            });

            if is_push {
                continue;
            }
            if stack_vec.is_empty() {
                return false;
            }
            let last = stack_vec[stack_vec.len() - 1];
            let is_pop = pairs.iter().any(|pair| {
                if x == pair.1 && last == pair.0 {
                    stack_vec.pop();
                    return true;
                }
                false
            });

            if is_pop {
                continue;
            }
            return false;
        }
        stack_vec.is_empty()
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
