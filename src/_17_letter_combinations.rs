///
/// [17. 电话号码的字母组合](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/)
///
struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let map = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut result = Vec::new();
        Self::append(
            0,
            &digits,
            &map,
            String::with_capacity(digits.len()),
            &mut result,
        );
        result
    }
    fn append(index: usize, digits: &str, map: &[&str], mut pre: String, result: &mut Vec<String>) {
        if index == digits.len() {
            result.push(pre);
            return;
        }
        let c = digits.chars().nth(index).unwrap();
        // char 转10进制 再转usize得到map对应的下标
        let i = c.to_digit(10).unwrap() as usize;
        if let Some(&letters) = map.get(i) {
            for letter in letters.chars() {
                pre.push(letter);
                Self::append(index + 1, digits, map, pre.clone(), result);
                pre.pop();
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let r = Solution::letter_combinations(String::from("23"));
        assert_eq!(
            r,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        )
    }
}
