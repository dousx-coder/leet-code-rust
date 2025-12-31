///
/// [1201. 丑数Ⅲ](https://leetcode.cn/problems/ugly-number-iii/)
///
struct Solution;
impl Solution {
    fn gcd(a: usize, b: usize) -> usize {
        if a % b == 0 { b } else { Self::gcd(b, a % b) }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a * b / Self::gcd(a, b)
    }

    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let n = n as usize;
        let a = a as usize;
        let b = b as usize;
        let c = c as usize;
        let mut low = 1;
        let mut high = 2_000_000_000;

        let ab = Self::lcm(a, b);
        let ac = Self::lcm(a, c);
        let bc = Self::lcm(b, c);
        let abc = Self::lcm(ab, c);

        while low < high {
            let k = low + (high - low) / 2;
            let mut cnt = 0;
            // 使用容斥原理计算小于等于k的丑数个数
            cnt += k / a + k / b + k / c + k / abc;
            cnt -= k / ab + k / ac + k / bc;

            if cnt < n {
                low = k + 1;
            } else if cnt == n {
                high = k;
            } else {
                high = k - 1;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::nth_ugly_number(3, 2, 3, 5), 4);
    }

    #[test]
    fn t2() {
        // 2, 3, 4, 6, 8, 9, 10, 12
        assert_eq!(Solution::nth_ugly_number(4, 2, 3, 4), 6);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::nth_ugly_number(5, 2, 11, 13), 10);
    }
}
