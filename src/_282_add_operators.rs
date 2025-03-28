/// 282 给表达式添加运算符
///
/// https://leetcode.cn/problems/expression-add-operators/description/
struct Solution;
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut chs = num.chars().collect::<Vec<char>>();
        let mut ans = vec![];
        Self::backtracking(0, 0, 0, target as i64, &mut vec![], &mut ans, &mut chs);
        ans
    }
    fn backtracking(
        index: usize,
        res: i64,
        mul: i64,
        target: i64,
        expr: &mut Vec<char>,
        ans: &mut Vec<String>,
        chs: &Vec<char>,
    ) {
        let len = chs.len();
        if index == len {
            if res == target {
                let expression = expr.iter().map(|s| s.to_string()).collect::<String>();
                ans.push(expression);
            }
            return;
        }
        let expr_len = expr.len();
        if index > 0 {
            expr.push('0');
        }
        let mut val: i64 = 0;
        let mut j = index;
        while j < len && (j == index || chs[index] != '0') {
            val = val * 10 + chs[j].to_digit(10).unwrap() as i64;
            expr.push(chs[j]);
            if index == 0 {
                Self::backtracking(j + 1, res + val, val, target, expr, ans, chs);
            } else {
                expr[expr_len] = '+';
                Self::backtracking(j + 1, res + val, val, target, expr, ans, chs);

                expr[expr_len] = '-';
                Self::backtracking(j + 1, res - val, -val, target, expr, ans, chs);

                expr[expr_len] = '*';
                Self::backtracking(
                    j + 1,
                    res - mul + mul * val,
                    mul * val,
                    target,
                    expr,
                    ans,
                    chs,
                );
                expr.pop();
            }
            j += 1;
        }
        while expr.len() > expr_len {
            expr.pop();
        }
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
