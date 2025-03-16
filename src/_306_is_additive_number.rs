/// 306 累加数
///
/// https://leetcode.cn/problems/additive-number/description/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let mut num = num.chars().collect::<Vec<char>>();
        Self::backtracking(0, &num, &mut vec![])
    }
    fn backtracking(start: usize, num: &Vec<char>, parse: &mut Vec<u128>) -> bool {
        let len = num.len();
        if start >= len {
            return false;
        }
        let range = if start < len && num[start] == '0' {
            start + 1
        } else {
            len
        };
        for i in start..range {
            // 这里有两种写法
            // 包含下一个0
            // 不包含下一个0 下一个作为独立的数字0
            let j = if i + 1 < len && num[i + 1] == '0' {
                i + 1
            } else {
                i
            };
            for end in i..=j {
                let c = Self::parse_num(start, num, end);
                parse.push(c);
                if parse.len() < 3 {
                    if Self::backtracking(end + 1, num, parse) {
                        return true;
                    } else {
                        parse.pop();
                    }
                    continue;
                }
                let a = parse[parse.len() - 3];
                let b = parse[parse.len() - 2];
                let c = parse[parse.len() - 1];

                if a + b < c {
                    // 剪枝
                    parse.pop();
                    break;
                }
                if a + b > c {
                    parse.pop();
                    continue;
                }
                if a + b == c {
                    if i == len - 1 {
                        return true;
                    };
                    if Self::backtracking(i + 1, num, parse) {
                        return true;
                    } else {
                        parse.pop();
                    }
                }
            }
        }
        false
    }

    fn parse_num(start: usize, num: &Vec<char>, i: usize) -> u128 {
        num[start..=i]
            .iter()
            .collect::<String>()
            .parse::<u128>()
            .unwrap()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(true, Solution::is_additive_number("112358".to_string()));
    }
    #[test]
    fn t2() {
        assert_eq!(true, Solution::is_additive_number("199100199".to_string()));
    }

    #[test]
    fn t3() {
        assert_eq!(false, Solution::is_additive_number("1023".to_string()));
    }

    #[test]
    fn t4() {
        assert_eq!(true, Solution::is_additive_number("101".to_string()));
    }
    #[test]
    fn t5() {
        assert_eq!(false, Solution::is_additive_number("011112".to_string()));
    }
    #[test]
    fn t6() {
        assert_eq!(true, Solution::is_additive_number("033".to_string()));
    }

    #[test]
    fn t7() {
        assert_eq!(true, Solution::is_additive_number("000".to_string()));
    }
    #[test]
    fn t8() {
        assert_eq!(true, Solution::is_additive_number("01212".to_string()));
    }
    #[test]
    fn t9() {
        assert_eq!(
            false,
            Solution::is_additive_number("19910011992".to_string())
        );
    }
}
