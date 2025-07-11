///
/// [2413. 最小偶倍数](https://leetcode.cn/problems/smallest-even-multiple/)
///
struct Solution;
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        (n % 2 + 1) * n
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }
}
