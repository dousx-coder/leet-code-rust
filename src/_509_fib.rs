/// 509 Fibonacci Number
///
/// https://leetcode.cn/problems/fibonacci-number/
struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let n = n as usize;
        let mut a = 0;
        let mut b = 1;
        let mut c = -1;
        for i in 2..=n {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::fib(2), 1);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::fib(3), 2);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::fib(4), 3);
    }
}
