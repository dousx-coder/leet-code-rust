///
///[738. 单调递增的数字](https://leetcode.cn/problems/monotone-increasing-digits/)
///
struct Solution;
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits = Vec::new();
        let mut num = n;

        // 将数字拆分为按位存储的数组
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();

        let mut flag = digits.len(); // 标记需要修改的位置

        // 检查是否存在非单调递增的情况
        for i in (1..digits.len()).rev() {
            if digits[i] < digits[i - 1] {
                flag = i; // 记录需要修改的位置
                digits[i - 1] -= 1; // 前一位减 1
            }
        }

        // 将标记位置及其之后的所有位设置为 9
        for i in flag..digits.len() {
            digits[i] = 9;
        }

        // 将数组重新组合为数字
        let mut result = 0;
        for &digit in &digits {
            result = result * 10 + digit;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
    }
    #[test]
    fn t4() {
        assert_eq!(Solution::monotone_increasing_digits(528357107), 499999999);
    }
}
