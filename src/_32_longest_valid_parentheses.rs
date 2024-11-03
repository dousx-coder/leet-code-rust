use std::cmp::max;
///
/// https://leetcode.cn/problems/longest-valid-parentheses/description/
struct Solution;
impl Solution {
    pub fn is_valid(s: &str) -> bool {
        let pairs = vec![('(', ')'), ('{', '}'), ('[', ']')];
        let mut stack_vec = Vec::new();
        for x in s.chars() {
            let is_push = pairs.iter()
                .any(|pair| {
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
            let is_pop = pairs.iter()
                .any(|pair| {
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

    pub fn recursion(parenthesis: &str, left: usize, right: usize) -> i32 {
        if left >= right {
            return 0;
        }
        let len = parenthesis.len();
        if len == 0 {
            return 0;
        }
        let pairs = ('(', ')');
        let slice = &parenthesis[left..=right];
        if slice.len() % 2 == 0 {
            if Solution::is_valid(slice) {
                return slice.len() as i32;
            }
        };
        let a = Solution::recursion(parenthesis, left + 1, right);
        let b = Solution::recursion(parenthesis, left, right - 1);
        max(a, b)
    }
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let len = s.len();
        if len == 0 {
            return 0;
        }
        Solution::recursion(&s, 0, len - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let r = Solution::longest_valid_parentheses(String::from("(()"));
        assert_eq!(r, 2);
    }

    #[test]
    fn t2() {
        let r = Solution::longest_valid_parentheses(String::from(")()())"));
        assert_eq!(r, 4);
    }
    #[test]
    fn t3() {
        let r = Solution::longest_valid_parentheses(String::from(""));
        assert_eq!(r, 0);
    }
    #[test]
    fn t4() {
        //  超时
        let r = Solution::longest_valid_parentheses(String::from(")(()(()(((())(((((()()))((((()()(()()())())())()))()()()())(())()()(((()))))()((()))(((())()((()()())((())))(())))())((()())()()((()((())))))((()(((((()((()))(()()(())))((()))()))())"));
        // println!("{r}")
    }
}