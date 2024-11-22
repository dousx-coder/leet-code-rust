struct Solution;
///
/// https://leetcode.cn/problems/longest-common-prefix/
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        // 公共前缀与所有的字符比,长度要么比所有字符都短要么长度等于最小的字符串长度
        let mut prefix = strs[0].clone();
        for str in strs {
            while !str.starts_with(&prefix) {
                // 删除最后一个字符
                prefix.pop();
            }
        }
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let result = Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!(result, "fl".to_string());
    }
    #[test]
    fn t2() {
        let result = Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ]);
        assert_eq!(result, "".to_string());
    }
}
