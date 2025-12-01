///
/// [172. 阶乘后的零](https://leetcode.cn/problems/factorial-trailing-zeroes/)
///
struct Solution;
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut count = 0;
        let mut n = n;
        while n >= 5 {
            count += n / 5;
            n /= 5;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::trailing_zeroes(5), 1);
    }
}
