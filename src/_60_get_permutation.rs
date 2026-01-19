use itertools::Itertools;

///
/// [60. 排列序列](https://leetcode.cn/problems/permutation-sequence/)
///
struct Solution;
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;

        // 转换为0索引
        let mut k = k as usize - 1;

        // 预计算阶乘
        let mut factorial = vec![1; n];
        for i in 1..n {
            factorial[i] = factorial[i - 1] * i;
        }

        let mut nums: Vec<usize> = (1..=n).collect();
        let mut result = String::new();

        // n的阶乘共有n * (n-1)!种组合
        // 以1-n开头的数字每一组有(n-1)!个数字 
        // k/(n-1)! 找到第几组，第k个数字属于第几组

        // 从高位开始构建结果
        for i in (0..n).rev() {
            // 确定当前位应该取nums中的哪个数
            let index = k / factorial[i];
            k %= factorial[i];

            result.push_str(&nums[index].to_string());
            // 移除已使用的数字
            nums.remove(index);
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::get_permutation(3, 1), "123".to_string());
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::get_permutation(3, 5), "312".to_string());
    }
}
