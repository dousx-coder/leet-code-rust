///
/// [319. 灯泡开关](https://leetcode.cn/problems/bulb-switcher/)
///
struct Solution;
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
    /// 暴力解法会超时
    fn violent(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let n: usize = n as usize;
        let mut status = vec![true; n];
        for setup in 2..=n {
            let mut j = setup - 1;
            while j < n {
                status[j] = !status[j];
                j += setup;
            }
        }

        status.into_iter().filter(|&x| x).count() as i32
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::violent(3), 1);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::bulb_switch(0), 0);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::bulb_switch(1), 1);
    }
    #[test]
    fn t4() {
        // 这个数据如果用暴力会超时
        // assert_eq!(Solution::violent(99999999), 9999);
        assert_eq!(Solution::bulb_switch(99999999), 9999);
    }
}
