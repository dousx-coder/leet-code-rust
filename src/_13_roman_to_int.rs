use std::collections::HashMap;
///
/// [13. 罗马数字转整数](https://leetcode.cn/problems/roman-to-integer/description/)
///
struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut prev = 0;

        // 使用 HashMap 存储字符到整数的映射
        let roman_map: HashMap<char, i32> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        // 从字符串末尾向前遍历
        for ch in s.chars().rev() {
            let x = roman_map.get(&ch).unwrap_or(&0);
            let curr = *x;
            if curr < prev {
                sum -= curr;
            } else {
                sum += curr;
            }
            prev = curr;
        }

        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }
    #[test]
    fn t4() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }
    #[test]
    fn t5() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
