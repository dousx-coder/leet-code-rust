///
/// 1047. 删除字符串中的所有相邻重复项
///
/// https://leetcode.cn/problems/remove-all-adjacent-duplicates-in-string/
struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let mut stack_vec = Vec::new();
        for c in s.chars() {
            if stack_vec.is_empty() {
                stack_vec.push(c);
                continue;
            }
            let pop = stack_vec.pop().unwrap();
            if c == pop {
                continue;
            }
            stack_vec.push(pop);
            stack_vec.push(c);
        }
        stack_vec.into_iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::remove_duplicates("abbaca".to_string()),
            "ca".to_string()
        );
    }
}
