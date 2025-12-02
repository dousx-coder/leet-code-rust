///
/// [204. 计数质数](https://leetcode.cn/problems/count-primes/?envType=problem-list-v2&envId=math)
///
struct Solution;
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut primes = vec![true; n];
        let mut count = 0;
        for i in 2..n {
            // 2是质数
            // 从2开始，如果当前数是质数，则从当前数开始，步长为当前数，将当前数之后的数都标记为非质数
            if primes[i] {
                count += 1;
                for j in (i * 2..n).step_by(i) {
                    primes[j] = false;
                }
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::count_primes(10), 4);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::count_primes(0), 0);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::count_primes(1), 0);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::count_primes(2), 0);
    }
}
