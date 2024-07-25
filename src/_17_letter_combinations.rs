use std::collections::HashMap;

///
/// 17. 电话号码的字母组合
///
/// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/
///
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if "" == digits {
            return vec![];
        }
        let mut map = HashMap::new();
        map.insert('2', vec!["a", "b", "c"]);
        map.insert('3', vec!["d", "e", "f"]);
        map.insert('4', vec!["g", "h", "i"]);
        map.insert('5', vec!["j", "k", "l"]);
        map.insert('6', vec!["m", "n", "o"]);
        map.insert('7', vec!["p", "q", "r", "s"]);
        map.insert('8', vec!["t", "u", "v"]);
        map.insert('9', vec!["w", "x", "y", "z"]);
        let mut r: Vec<String> = vec![];
        Solution::append(0, &digits, &map, "", &mut r);
        r
    }
    fn append(
        index: usize,
        digits: &str,
        map: &HashMap<char, Vec<&str>>,
        pre: &str,
        result: &mut Vec<String>,
    ) {
        if index == digits.len() {
            result.push(String::from(pre));
            return;
        }
        let mut char_indices = digits.char_indices();
        match char_indices.nth(index).map(|(_, c)| c) {
            Some(c) => {
                let option_v = map.get(&c);
                let ref_str_vec = option_v.unwrap();
                ref_str_vec.iter().for_each(|&it| {
                    let s = pre.to_string() + it;
                    let sp = &s;
                    Solution::append(index + 1, digits, map, sp, result)
                });
            }
            None => {}
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
