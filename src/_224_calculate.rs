use std::collections::HashMap;

///
/// `224. 基本计算器`
///
/// https://leetcode.cn/problems/basic-calculator/
///

struct Solution;
impl Solution {
    ///
    /// 求逆波兰式的值
    ///
    fn eval_rpn(rpn: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        let has_op = rpn
            .iter()
            .any(|it| it == "+" || it == "-" || it == "*" || it == "/");
        if !has_op {
            return rpn.join("").parse::<i32>().unwrap();
        }
        for token in rpn {
            let is_operator = token == "+" || token == "-" || token == "*" || token == "/";
            if !is_operator || stack.is_empty() {
                stack.push(token);
                continue;
            }
            let num1 = stack.pop().unwrap().to_string().parse::<i32>().unwrap();
            let num2 = stack.pop().unwrap().to_string().parse::<i32>().unwrap();
            let ans = match token.as_str() {
                "+" => num2 + num1,
                "-" => num2 - num1,
                "*" => num2 * num1,
                "/" => num2 / num1,
                _ => {
                    panic!()
                }
            };
            stack.push(ans.to_string());
        }
        stack.pop().unwrap().to_string().parse::<i32>().unwrap()
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
        let vec = s
            .chars()
            .into_iter()
            .filter(|c| *c != ' ')
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        // 后缀式(逆波兰式)
        let mut rpn_vec = Vec::new();
        // 运算符栈
        let mut operator_stack = Vec::new();
        for curr_token in vec {
            if curr_token == "(" {
                operator_stack.push(curr_token);
                continue;
            }
            if curr_token == ")" {
                loop {
                    //取出所有字符追加到逆波兰式中
                    let top = operator_stack.pop().unwrap();
                    if top == "(" {
                        break;
                    }
                    rpn_vec.push(top);
                }
                continue;
            }
            if curr_token == "+" || curr_token == "-" || curr_token == "*" || curr_token == "/" {
                if operator_stack.is_empty() {
                    operator_stack.push(curr_token);
                } else {
                    let curr_precedence = Self::operator_precedence(&curr_token);
                    loop {
                        let top = operator_stack.pop().unwrap();
                        if top == "(" || Self::operator_precedence(&top) < curr_precedence {
                            operator_stack.push(top);
                            operator_stack.push(curr_token);
                            break;
                        } else {
                            rpn_vec.push(top);
                        }
                        if operator_stack.is_empty() {
                            operator_stack.push(curr_token);
                            break;
                        }
                    }
                }
                continue;
            }
            // 数字 追加到逆波兰式
            rpn_vec.push(curr_token);
        }
        while !operator_stack.is_empty() {
            rpn_vec.push(operator_stack.pop().unwrap());
        }
        rpn_vec
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
}
