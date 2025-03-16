/// 842 将数组拆分成斐波那契序列
///
/// https://leetcode.cn/problems/split-array-into-fibonacci-sequence/
struct Solution;
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut num = num.chars().collect::<Vec<char>>();
        let mut parse = vec![];
        if Self::backtracking(0, &mut parse, &num) {
            let mut result = vec![];
            for i in 0..parse.len() {
                let x = parse[i];
                let it = num[x.0..=x.1]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                result.push(it);
            }
            result
        } else {
            vec![]
        }
    }
    fn backtracking(start: usize, parse: &mut Vec<(usize, usize)>, num: &Vec<char>) -> bool {
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
                let x = num[start..=end].iter().collect::<String>();
                let num_max_str = i32::MAX.to_string();
                if x.len() >= num_max_str.len() && x > num_max_str {
                    // 由于返回值的限制是i32，所以这里要做剪枝判断
                    // 如果当前切片不是最后一个数字，则不能大于i32最大值
                    break;
                }
                parse.push((start, end));
                if parse.len() < 3 {
                    if Self::backtracking(end + 1, parse, num) {
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
                    if Self::backtracking(i + 1, parse, num) {
                        return true;
                    } else {
                        parse.pop();
                    }
                }
            }
        }
        false
    }
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
        assert_eq!(
            vec![11, 0, 11, 11],
            Solution::split_into_fibonacci("1101111".to_string())
        );
    }
    #[test]
    fn t2() {
        let  ans = Solution::split_into_fibonacci("539834657215398346785398346991079669377161950407626991734534318677529701785098211336528511".to_string());
        assert!(ans.is_empty())
    }
    #[test]
    fn t3() {
        assert_eq!(
            vec![123, 456, 579],
            Solution::split_into_fibonacci("123456579".to_string())
        );
    }
}
