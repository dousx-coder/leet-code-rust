/// 282 给表达式添加运算符
///
/// https://leetcode.cn/problems/expression-add-operators/description/
struct Solution;
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut chs = num.chars().collect::<Vec<char>>();
        let mut ans = vec![];
        let operator = vec!["+", "-", "*"];
        Self::backtracking(0, target, &mut vec![], &mut ans, &chs, &operator);
        ans
    }
    fn backtracking(
        index: usize,
        target: i32,
        path: &mut Vec<String>,
        ans: &mut Vec<String>,
        chs: &Vec<char>,
        operator: &Vec<&str>,
    ) {
        let len = chs.len();
        for i in index..len {
            let index_is_zero = chs[index] == '0';
            let num = if index_is_zero {
                "0".to_string()
            } else {
                chs[index..i + 1].iter().collect::<String>()
            };
            path.push(num);
            if i == len - 1 {
                let pl = path.len();
                if pl >= 3 {
                    // 长度大于3 且倒数第二个是运算发式，则计算
                    match path[pl - 2].clone().as_str() {
                        "+" | "-" | "*" | "/" => {
                            // 最后一个数字
                            let cal = Self::evaluate_expression(path);
                            if cal == target {
                                let expression =
                                    path.iter().map(|s| s.to_string()).collect::<String>();
                                ans.push(expression);
                            }
                        }
                        _ => {}
                    }
                }
                // 弹出最后一个数字，回溯到上层也会弹出操作符
                path.pop();
                return;
            } else {
                for x in operator {
                    path.push((*x).to_string());
                    Self::backtracking(i + 1, target, path, ans, chs, operator);
                    path.pop();
                }
                // 弹出当前数字
                path.pop();
                if index_is_zero {
                    break;
                }
            }
        }
    }

    ///
    /// 求逆波兰式的值
    ///
    fn eval_rpn(rpn: &Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in rpn {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    if stack.is_empty() {
                        panic!("Unexpected empty rpn");
                    }
                    let ans = if stack.len() == 1 {
                        // 栈中只有一个值，说明应该是 负数写法
                        let num = stack.pop().unwrap();
                        match token.as_str() {
                            "+" => 0 + num,
                            "-" => 0 - num,
                            _ => panic!("不支持的操作"),
                        }
                    } else {
                        let num1 = stack.pop().unwrap();
                        let num2 = stack.pop().unwrap();
                        match token.as_str() {
                            "+" => num2 + num1,
                            "-" => num2 - num1,
                            "*" => num2 * num1,
                            "/" => num2 / num1,
                            _ => unreachable!(),
                        }
                    };

                    stack.push(ans);
                }
                _ => match token.parse::<i32>() {
                    Ok(num) => {
                        stack.push(num);
                    }
                    Err(_) => {}
                },
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
    fn conversion_rpn(tokens: &Vec<String>) -> Vec<String> {
        let mut rpn_vec = Vec::new();
        // 运算符栈
        let mut operator_stack = Vec::new();
        for token in tokens {
            match token.as_str() {
                "(" => operator_stack.push(token),
                ")" => {
                    while let Some(top) = operator_stack.pop() {
                        if top == "(" {
                            break;
                        }
                        rpn_vec.push(top.to_string());
                    }
                }
                "+" | "-" | "*" | "/" => {
                    while operator_stack.last().map_or(false, |top| {
                        *top != "("
                            && Self::operator_precedence(top.as_str())
                                >= Self::operator_precedence(&token)
                    }) {
                        rpn_vec.push(operator_stack.pop().unwrap().to_string());
                    }
                    operator_stack.push(token);
                }
                // 数字直接追加
                _ => rpn_vec.push(token.to_string()),
            }
        }
        while let Some(op) = operator_stack.pop() {
            rpn_vec.push(op.to_string());
        }
        rpn_vec
    }

    ///
    /// 计算表达式的值
    ///
    fn evaluate_expression(expr: &Vec<String>) -> i32 {
        let rpn = Self::conversion_rpn(expr);
        Self::eval_rpn(&rpn)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn test1() {
        let expected = hashset! {"1+2+3".to_string(), "1*2*3".to_string()};
        let ans = Solution::add_operators("123".to_string(), 6);
        assert_eq!(expected.len(), ans.len());
        assert_eq!(expected, HashSet::from_iter(ans.into_iter()));
    }

    #[test]
    fn test2() {
        let expected = hashset! {"2*3+2".to_string(), "2+3*2".to_string()};
        let ans = Solution::add_operators("232".to_string(), 8);
        assert_eq!(expected.len(), ans.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }

    #[test]
    fn test3() {
        let ans = Solution::add_operators("3456237490".to_string(), 9191);
        assert!(ans.is_empty());
    }
    #[test]
    fn test4() {
        let expected = hashset! {"1*0+5".to_string(), "10-5".to_string()};
        let ans = Solution::add_operators("105".to_string(), 5);
        assert_eq!(expected.len(), ans.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }
}
