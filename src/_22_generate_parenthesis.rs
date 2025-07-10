///
/// [22. 括号生成](https://leetcode.cn/problems/generate-parentheses/description/)
///
struct Solution;
impl Solution {
    fn dfs(n: i32, left: i32, right: i32, append: &str, result: &mut Vec<String>) {
        if left == right && left == n {
            result.push(append.to_string());
            return;
        }
        if left < n {
            let s = &format!("{append}(");
            Self::dfs(n, left + 1, right, s, result);
        }
        if right < n && right + 1 <= left {
            let s = &format!("{append})");
            Self::dfs(n, left, right + 1, s, result);
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Self::dfs(n, 0, 0, "", &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let result = Solution::generate_parenthesis(1);
        assert_eq!(result, vec!["()".to_string()]);
    }
    #[test]
    fn t2() {
        let result = Solution::generate_parenthesis(3);
        assert_eq!(
            result,
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ]
        );
    }
}
