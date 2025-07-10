///
/// [50. Pow(x, n)](https://leetcode.cn/problems/powx-n/description/)
///
struct Solution;
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut num = x;
        let mut ans: f64 = 1.0;
        let mut j = n.abs();
        while j != 0 {
            if j % 2 != 0 {
                ans *= num;
            }
            num *= num;
            j /= 2;
        }
        // 如果存在小数点，这里返回值并不是100%精确
        if n < 0 { 1.0 / ans } else { ans }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::my_pow(2.00000, 10);
        assert_eq!(ans, 1024.00000);
    }
    #[test]
    fn t2() {
        let ans = Solution::my_pow(2.0, -2);
        assert_eq!(ans, 0.25);
    }

    #[test]
    fn t3() {
        let ans = Solution::my_pow(2.10000, 3);
        assert!((ans - 9.26100).abs() < 1e-9);
    }
}
