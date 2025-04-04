/// 301 删除无效的括号
///
/// https://leetcode.cn/problems/remove-invalid-parentheses/description/
struct Solution;
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut chs = s.chars().collect::<Vec<char>>();
        let (lr, rr) = Self::remove_count(&chs);
        let mut ans = vec![];
        Self::backtracking(&mut ans, &mut chs, lr, rr, 0);
        ans
    }

    /// 计算需要移除括号的数量
    fn remove_count(chs: &Vec<char>) -> (i32, i32) {
        // 需要移除的括号数量
        let mut lr = 0;
        let mut rr = 0;
        for i in 0..chs.len() {
            let x = chs[i];
            if x == '(' {
                lr += 1;
            } else if x == ')' {
                if lr > 0 {
                    // 遇到右括号 抵消1次左括号
                    lr -= 1;
                } else {
                    // 需要移除的右括号+1
                    rr += 1;
                }
            }
        }
        (lr, rr)
    }

    /// 回溯删除括号
    fn backtracking(ans: &mut Vec<String>, chs: &mut Vec<char>, lr: i32, rr: i32, start: usize) {
        if lr == 0 && rr == 0 {
            if Solution::is_valid(chs) {
                ans.push(chs.iter().collect::<String>());
            }
            return;
        }
        for i in start..chs.len() {
            if i > start && chs[i] == chs[i - 1] {
                // 去重
                continue;
            }
            if chs[i] == '(' && lr > 0 {
                // 删除当前字符
                let c = chs[i];
                chs.remove(i);
                // 下一个start传递的仍然是i,因为chs[i]被删除了
                Solution::backtracking(ans, chs, lr - 1, rr, i);
                chs.insert(i, c);
            }
            if chs[i] == ')' && rr > 0 {
                // 删除当前字符
                let c = chs[i];
                chs.remove(i);
                Solution::backtracking(ans, chs, lr, rr - 1, i);
                chs.insert(i, c);
            }
        }
    }

    /// 校验括号是否合法
    fn is_valid(chs: &Vec<char>) -> bool {
        // 使用左括号数量替代栈
        let mut left = 0;
        for x in chs {
            if *x == '(' {
                left += 1;
                continue;
            }
            if *x == ')' {
                // 遇到右括号 抵消1次左括号
                left -= 1;
                if left < 0 {
                    // 右括号比左括号多
                    return false;
                }
            }
        }
        left == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let ans = Solution::remove_invalid_parentheses("()())()".to_string());
        let exp = hashset! {"(())()", "()()()"};
        assert_eq!(ans.len(), exp.len());
        let ans = ans.iter().map(|s| s.as_str()).collect::<HashSet<_>>();
        assert_eq!(ans, exp);
    }
    #[test]
    fn t2() {
        let ans = Solution::remove_invalid_parentheses("(a)())()".to_string());
        let exp = hashset! {"(a())()","(a)()()"};
        assert_eq!(ans.len(), exp.len());
        let ans = ans.iter().map(|s| s.as_str()).collect::<HashSet<_>>();
        assert_eq!(ans, exp);
    }
    #[test]
    fn t3() {
        let ans = Solution::remove_invalid_parentheses("".to_string());
        assert_eq!(ans, vec!["".to_string()]);
    }

    #[test]
    fn t4() {
        let ans = Solution::remove_invalid_parentheses(")(".to_string());
        assert_eq!(ans, vec!["".to_string()]);
    }
}
