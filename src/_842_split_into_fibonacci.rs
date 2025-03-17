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
            return parse.len() >= 3;
        }
        let range = if start < len && num[start] == '0' {
            start + 1
        } else {
            len
        };
        for end in start..range {
            let x = num[start..=end].iter().collect::<String>();
            // 10 是i32最大值长度
            if x.len() > 10 || x.parse::<i32>().is_err() {
                break;
            }
            parse.push((start, end));
            if parse.len() < 3 || Self::eq(num, parse) == 0 {
                if Self::backtracking(end + 1, parse, num) {
                    return true;
                }
            }
            parse.pop();
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
        let b = parse[parse_len - 2];
        let c = parse[parse_len - 1];
        // 这道题的返回值是i32数组，所以最大值不会超过i32，这里直接转i64相加比自己实现加法判断要快
        let a: i64 = num[a.0..=a.1].iter().collect::<String>().parse().unwrap();
        let b: i64 = num[b.0..=b.1].iter().collect::<String>().parse().unwrap();
        let c: i64 = num[c.0..=c.1].iter().collect::<String>().parse().unwrap();
        if a + b == c {
            0
        } else if a + b < c {
            -1
        } else {
            1
        }
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
