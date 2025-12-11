///
/// [400. 第N位数字](https://leetcode.cn/problems/nth-digit/?envType=problem-list-v2&envId=binary-search)
///
/// 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 N 拼起来的字符串找第N位是什么
///
struct Solution;
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        // 使用 i64 避免溢出
        let mut n = n as i64;
        // 当前考虑的数字位数（1位、2位...）
        let mut digits = 1;
        // 当前位数下的总数字数量（如 1 位数有 9 个）
        let mut count = 9;

        // 找到目标所在的位数段
        while n > digits * count {
            n -= digits * count;
            digits += 1;
            count *= 10;
        }

        // 计算目标数字是该段中的第几个数
        let index = (n - 1) / digits;
        let digit_index = ((n - 1) % digits) as usize;

        // 得到起始数字并加上偏移量得到实际数字
        let start = 10_i64.pow(digits as u32 - 1);
        let num = start + index;

        // 取出对应位置上的数字
        num.to_string()
            .chars()
            .nth(digit_index)
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        // 123
        assert_eq!(Solution::find_nth_digit(3), 3);
    }

    #[test]
    fn t2() {
        // 12345678910
        assert_eq!(Solution::find_nth_digit(11), 0);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::find_nth_digit(1000000000), 1);
    }

    #[test]
    fn t4() {
        // 123456789101112
        assert_eq!(Solution::find_nth_digit(15), 2);
    }
}
