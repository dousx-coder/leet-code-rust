/// `150. 逆波兰表达式求值`
///
/// https://leetcode.cn/problems/evaluate-reverse-polish-notation/
///
struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in tokens {
            let is_operator = token == "+" || token == "-" || token == "*" || token == "/";
            if !is_operator || stack.is_empty() {
                stack.push(token);
                continue;
            }
            let num1 = stack.pop().unwrap().to_string().parse::<i32>().unwrap();
            let num2 = stack.pop().unwrap().to_string().parse::<i32>().unwrap();
            let ans = match token.as_str() {
                "+" => num1 + num2,
                "-" => num2 - num1,
                "*" => num1 * num2,
                "/" => num2 / num1,
                _ => {
                    panic!()
                }
            };
            stack.push(ans.to_string());
        }
        stack.pop().unwrap().to_string().parse::<i32>().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let vec = vec!["2", "1", "+", "3", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Solution::eval_rpn(vec), 9)
    }

    #[test]
    fn t2() {
        let vec = vec!["4", "13", "5", "/", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let ans = Solution::eval_rpn(vec);
        assert_eq!(ans, 6)
    }

    #[test]
    fn t3() {
        let vec = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let ans = Solution::eval_rpn(vec);
        assert_eq!(ans, 22)
    }

    #[test]
    fn t4() {
        let vec = vec![
            "4", "3", "-",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let ans = Solution::eval_rpn(vec);
        assert_eq!(ans, 1)
    }
}
