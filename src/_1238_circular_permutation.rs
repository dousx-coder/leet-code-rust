use itertools::Itertools;
///
/// [1238. 循环码排列 (89 格雷编码)](https://leetcode.cn/problems/circular-permutation-in-binary-representation/description/)
///
/// [89. 格雷编码](https://leetcode.cn/problems/gray-code/)
///
struct Solution;
impl Solution {
    ///
    ///  ```txt
    /// 关键是搞清楚格雷编码的生成过程, G(i) = i ^ (i/2);
    ///        如 n = 3:
    ///
    ///        G(0) = 000,
    ///
    ///        G(1) = 1 ^ 0 = 001 ^ 000 = 001
    ///
    ///        G(2) = 2 ^ 1 = 010 ^ 001 = 011
    ///
    ///        G(3) = 3 ^ 1 = 011 ^ 001 = 010
    ///
    ///        G(4) = 4 ^ 2 = 100 ^ 010 = 110
    ///
    ///        G(5) = 5 ^ 2 = 101 ^ 010 = 111
    ///
    ///        G(6) = 6 ^ 3 = 110 ^ 011 = 101
    ///
    ///        G(7) = 7 ^ 3 = 111 ^ 011 = 100
    /// ```
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut ret = vec![start];
        for i in 1..=n {
            let len = ret.len();
            for j in (0..=len - 1).rev() {
                let num = ((ret[j] ^ start) | (1 << (i - 1))) ^ start;
                ret.push(num);
            }
        }
        ret
    }

    fn verify(ans: &Vec<i32>) -> bool {
        let a = ans[0];
        let b = ans[ans.len() - 1];
        if !Self::only_one_bit_diff(a, b) {
            return false;
        }
        // 1 <= n <= 16 这里转二进制取16位(对齐)
        for i in 1..ans.len() {
            if !Self::only_one_bit_diff(ans[i - 1], ans[i]) {
                return false;
            }
        }
        true
    }

    fn only_one_bit_diff(a: i32, b: i32) -> bool {
        let ba = format!("{:016b}", a).chars().collect_vec();
        let bb = format!("{:016b}", b).chars().collect_vec();
        let mut diff = 0;
        for i in 0..ba.len() {
            if ba[i] != bb[i] {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
        }
        diff == 1
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::circular_permutation(2, 3);
        assert!(vec![3, 1, 0, 2] == ans || vec![3, 2, 0, 1] == ans);
    }
    #[test]
    fn t2() {
        let ans = Solution::circular_permutation(3, 2);
        assert!(Solution::verify(&ans));
    }

    #[test]
    fn t3() {
        let ans = Solution::circular_permutation(4, 1);
        assert!(Solution::verify(&ans));
    }
}
