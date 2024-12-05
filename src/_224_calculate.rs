use std::collections::HashMap;

///
/// `224. 基本计算器`
///
/// https://leetcode.cn/problems/basic-calculator/
///
///
struct Solution;

impl Solution {
    ///
    /// 求逆波兰式的值
    ///
    fn eval_rpn(rpn: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in rpn {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let num1 = stack.pop().unwrap();
                    let num2 = stack.pop().unwrap();
                    let ans = match token.as_str() {
                        "+" => num2 + num1,
                        "-" => num2 - num1,
                        "*" => num2 * num1,
                        "/" => num2 / num1,
                        _ => unreachable!(),
                    };
                    stack.push(ans);
                }
                _ => {
                    stack.push(token.parse::<i32>().unwrap());
                }
            }
        }
        stack.pop().unwrap()
    }
    ///
    /// 运算符优先级
    ///
    fn operator_precedence(op: &str) -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => panic!("Invalid operator"),
        }
    }
    ///
    /// 将中缀式转为后缀式(逆波兰式)
    ///
    fn conversion_rpn(s: String) -> Vec<String> {
        let mut rpn_vec = Vec::new();
        // 运算符栈
        let mut operator_stack = Vec::new();
        let tokens = Self::tokenize(&s);
        for token in tokens {
            match token.as_str() {
                "(" => operator_stack.push(token),
                ")" => {
                    while let Some(top) = operator_stack.pop() {
                        if top == "(" {
                            break;
                        }
                        rpn_vec.push(top);
                    }
                }
                "+" | "-" | "*" | "/" => {
                    while operator_stack.last().map_or(false, |top| {
                        top != "("
                            && Self::operator_precedence(top) >= Self::operator_precedence(&token)
                    }) {
                        rpn_vec.push(operator_stack.pop().unwrap());
                    }
                    operator_stack.push(token);
                }
                // 数字直接追加
                _ => rpn_vec.push(token),
            }
        }
        while let Some(op) = operator_stack.pop() {
            rpn_vec.push(op);
        }
        rpn_vec
    }
    ///
    /// 将字符串拆解成数字和符号
    fn tokenize(s: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut num = String::new();
        let mut chars = s.chars().peekable();
        while let Some(&c) = chars.peek() {
            match c {
                ' ' => {
                    chars.next();
                }
                '(' | ')' => {
                    if !num.is_empty() {
                        tokens.push(num.clone());
                        num.clear();
                    }
                    tokens.push(c.to_string());
                    chars.next();
                }
                '+' | '*' | '/' => {
                    if !num.is_empty() {
                        tokens.push(num.clone());
                        num.clear();
                    }
                    tokens.push(c.to_string());
                    chars.next();
                }
                '-' => {
                    if !num.is_empty() {
                        tokens.push(num.clone());
                        num.clear();
                        // 当前是减号
                        tokens.push("-".to_string());
                        chars.next();
                    } else {
                        // 检查是否是负号
                        if tokens.is_empty()
                            || matches!(
                                tokens.last().unwrap().as_str(),
                                "(" | "+" | "-" | "*" | "/"
                            )
                        {
                            // 作为负数前缀
                            num.push(c);
                            chars.next();
                        } else {
                            tokens.push("-".to_string());
                            chars.next();
                        }
                    }
                }
                _ if c.is_digit(10) => {
                    num.push(c);
                    chars.next();
                }
                _ => {
                    panic!("Unexpected character");
                }
            }
        }
        if !num.is_empty() {
            tokens.push(num);
        }
        tokens
    }
    ///
    /// [`s`] 由数字、'+'、'-'、'('、')'、和 ' ' 组成
    ///
    pub fn calculate(s: String) -> i32 {
        let rpn = Self::conversion_rpn(s);
        Self::eval_rpn(rpn)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        // 2 3 5 + * 7 1 / + 4 -
        let ans = Solution::calculate(String::from("2 * ( 3 + 5 ) + 7 / 1 - 4"));
        assert_eq!(ans, 19);
    }

    #[test]
    fn t2() {
        let ans = Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)"));
        assert_eq!(ans, 23);
    }
    #[test]
    fn t3() {
        let ans = Solution::calculate(String::from("2147483647"));
        assert_eq!(ans, 2147483647);
    }
    #[test]
    fn t4() {
        let ans = Solution::calculate(String::from("1-(     -2)"));
        println!("{}", ans);
        assert_eq!(ans, 3);
    }

    #[test]
    fn t5() {
        let ans = Solution::calculate(String::from("- (3 + (4 + 5))"));
        assert_eq!(ans, -12);
    }
}
