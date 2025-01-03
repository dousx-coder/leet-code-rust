struct Solution;
///
/// 29 两数相除
/// https://leetcode.cn/problems/divide-two-integers/description/
///
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let flag = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0);
        let mut res = 0;
        let mut dsor;
        let mut num;
        let mut dividend = if dividend < 0 { dividend } else { -dividend };
        let mut divisor = if divisor < 0 { divisor } else { -divisor };
        while dividend <= divisor {
            dsor = divisor;
            num = -1;
            while dividend - dsor <= dsor {
                dsor <<= 1;
                num <<= 1;
            }
            res += num;
            dividend -= dsor;
        }
        if flag {
            res
        } else {
            -res
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }
}
