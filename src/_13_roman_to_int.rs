struct Solution;
///
/// https://leetcode.cn/problems/roman-to-integer/description/
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut prev = 0;
        let str_list = s
            .chars()
            .into_iter()
            .map(|c| c)
            .rev()
            .collect::<Vec<char>>();
        for x in str_list {
            let curr = Self::char_to_i32(x);
            if curr < prev {
                sum -= curr;
            } else {
                sum += curr;
            }
            prev = curr;
        }
        sum
    }
    fn char_to_i32(ch: char) -> i32 {
        if ch == 'I' {
            1
        } else if ch == 'V' {
            5
        } else if ch == 'X' {
            10
        } else if ch == 'L' {
            50
        } else if ch == 'C' {
            100
        } else if ch == 'D' {
            500
        } else if ch == 'M' {
            1000
        } else {
            0
        }
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
