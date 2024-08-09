use std::collections::HashMap;

struct Solution {}

impl Solution {
    /// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
    /// 思路滑动窗口
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let bytes = s.as_bytes();
        let mut start: i32 = 0;
        let mut ans: i32 = 0;
        for (i, &item) in bytes.iter().enumerate() {
            let character = item as char;
            if map.contains_key(&character) {
                let mv = *map.get(&character).unwrap();
                if start < mv {
                    start = mv;
                };
            }
            let index = i as i32;
            map.insert(character, index + 1);
            let temp = index - start + 1;
            if ans < temp {
                ans = temp;
            };
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_longest_substring(String::from("c")), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::length_of_longest_substring(String::from("au")), 2);
    }
}
