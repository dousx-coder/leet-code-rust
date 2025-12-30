///
/// [264 丑数 II](https://leetcode.cn/problems/ugly-number-ii/)
///
struct Solution;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n <= 0 {
            panic!("n <= 0");
        }
        if n == 1 {
            return 1;
        }
        let n = n as usize;
        // 动态规划 + 三指针

        // 三三比较谁最小，谁小谁就入丑表，
        // 入了丑表拿来乘，继续乘以二三五，
        // 再比大小入丑表，丑表满格见成效。
        // [1] (1x2, 1x3, 1x5) => [1, 2] (2x2, 1x3, 1x5) => [1, 2, 3] (2x2, 2x3, 1x5) => [1, 2, 3, 4] (3x2, 2x3, 1x5) ....
        let mut ugly = vec![1; n];
        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;
        for i in 1..n {
            let next2 = ugly[p2] * 2;
            let next3 = ugly[p3] * 3;
            let next5 = ugly[p5] * 5;
            let min = next2.min(next3).min(next5);
            if min == next2 {
                p2 += 1;
            }
            if min == next3 {
                p3 += 1;
            }
            if min == next5 {
                p5 += 1;
            }
            ugly[i] = min;
        }
        ugly[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::nth_ugly_number(1), 1);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::nth_ugly_number(1690), 2123366400);
    }
}
