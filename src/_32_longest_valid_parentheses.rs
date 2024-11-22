use std::cmp::max;

///
/// https://leetcode.cn/problems/longest-valid-parentheses/description/
struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let len = s.len();
        if len == 0 {
            return 0;
        }
        let mut stack = vec![-1];
        let mut max_len = 0;
        // 使用栈记录每一个未匹配的括号索引，当遇到匹配的括号时，计算它与最近未匹配括号之间的距离，即为有效括号的长度。
        for (i, ch) in s.chars().enumerate() {
            let index = i as i32;
            if ch == '(' {
                stack.push(index);
            } else {
                stack.pop();
                if let Some(&last) = stack.last() {
                    max_len = max(max_len, index - last);
                } else {
                    stack.push(index);
                }
            }
        }
        max_len
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
        assert_eq!(r, 132);
    }
}
