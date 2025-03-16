/// 306 累加数
///
/// https://leetcode.cn/problems/additive-number/description/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let mut num = num.chars().collect::<Vec<char>>();
        Self::backtracking(0, &num, &mut vec![])
    }
    fn backtracking(start: usize, num: &Vec<char>, parse: &mut Vec<(usize, usize)>) -> bool {
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
                let c = (start, end);
                parse.push((start, end));
                if parse.len() < 3 {
                    if Self::backtracking(end + 1, num, parse) {
                        return true;
                    } else {
                        parse.pop();
                    }
                    continue;
                }
                let check = Self::eq(num, parse);
                if check < 0 {
                    // 剪枝
                    parse.pop();
                    break;
                }
                if check > 0 {
                    parse.pop();
                    continue;
                }
                if check == 0 {
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
    /// a+b和c大小判断 (u128长度适用于大部分测试用例,自定义比较适用于任意长度的测试用例)
    ///
    /// -1: a + b < c
    ///
    /// 0 : a + b = c
    ///
    /// 1 : a + b > c (剪枝)
    fn eq(num: &Vec<char>, parse: &mut Vec<(usize, usize)>) -> i8 {
        let parse_len = parse.len();
        let a = parse[parse_len - 3];
        let a = &num[a.0..=a.1];

        let b = parse[parse_len - 2];
        let b = &num[b.0..=b.1];

        let c = parse[parse_len - 1];
        let c = &num[c.0..=c.1];
        // 进位
        let mut carry = 0;
        let mut sum = vec![];
        let mut ia = (a.len() - 1) as i32;
        let mut ib = (b.len() - 1) as i32;
        while ia >= 0 || ib >= 0 {
            let add1 = if ia < 0 {
                0
            } else {
                a[ia as usize].to_digit(10).unwrap() as u8
            };
            let add2 = if ib < 0 {
                0
            } else {
                b[ib as usize].to_digit(10).unwrap() as u8
            };
            ia -= 1;
            ib -= 1;
            let added = add1 + add2 + carry;
            sum.push(added % 10);
            carry = added / 10;
        }
        if carry != 0 {
            sum.push(carry);
        }
        let len = sum.len();
        let c_len = c.len();
        if len < c_len {
            return -1;
        }
        if len > c_len {
            return 1;
        }
        for i in (0..=len - 1).rev() {
            let x = c[len - 1 - i].to_digit(10).unwrap() as u8;
            if sum[i] != x {
                return if sum[i] > x { 1 } else { -1 };
            }
        }
        0
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
