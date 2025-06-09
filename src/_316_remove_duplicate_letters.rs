/// 316 去除重复字母
///
/// https://leetcode.cn/problems/remove-duplicate-letters/?envType=problem-list-v2&envId=stack
use std::collections::HashSet;
use std::hash::Hash;

struct Solution;
impl Solution {
    /// 单调栈
    pub fn remove_duplicate_letters(s: String) -> String {
        // 统计字符出现次数
        let mut count = [0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        // 栈用于维护结果顺序
        let mut stack = vec![];
        // 集合记录已存在的字符
        let mut existed = HashSet::new();
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            count[idx] -= 1; // 当前字符计数减1
            if existed.contains(&c) {
                // 已存在则跳过
                continue;
            }
            // 栈顶元素大于当前字符且后续仍存在时弹出
            while let Some(&top) = stack.last() {
                let top_idx = (top as u8 - b'a') as usize;
                if top > c && count[top_idx] > 0 {
                    existed.remove(&top);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(c);
            existed.insert(c);
        }
        stack.iter().collect()
    }

    /// 递归解法
    fn recursive(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }
        let mut unique_chars: Vec<char> = s.chars().collect();
        unique_chars.sort_unstable();
        //dedup 删除 vector 中的连续重复元素
        unique_chars.dedup();

        for &ch in &unique_chars {
            if let Some(idx) = s.find(ch) {
                let tmp = &s[idx..];
                // Check if all unique characters are present in `tmp`
                let tmp_unique = tmp.chars().collect::<HashSet<char>>();
                let s_unique = s.chars().collect::<HashSet<char>>();
                if tmp_unique == s_unique {
                    let remaining = tmp.replace(ch, "");
                    return ch.to_string() + &Self::recursive(remaining);
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string()),
            "abc"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string()),
            "acdb"
        );
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::recursive("cbacdcbc".to_string()), "acdb");
    }
}
