///
/// [43. 字符串相乘](https://leetcode.cn/problems/multiply-strings/description/)
///
struct Solution;
impl Solution {
    ///
    ///```text
    /// 思路如下：
    ///               9   9   9    num1
    ///         ×     6   7   8    num2
    ///  ----------------------
    ///              72  72  72
    ///          63  63  63
    ///      54  54  54
    ///  ----------------------
    ///      54 117 189 135  72
    ///  ----------------------
    ///      54 117 189 142   2
    ///  -----------------------
    ///      54 117 203   2   2
    ///  -----------------------
    ///      54 137   3   2   2
    ///  -----------------------
    ///      67   7   3   2   2
    ///  -----------------------
    ///   6   7   7   3   2   2
    /// ```
    ///
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        if num1.parse::<i32>().unwrap() < 0 || num2.parse::<i32>().unwrap() < 0 {
            // 不支持负数
            panic!("Illegal Argument: negative")
        };
        let num1 = num1
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let num2 = num2
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let len1 = num1.len();
        let len2 = num2.len();
        // 宽度
        let width = len1 + len2 - 1;
        // 高度
        let height = len2;
        let mut mul_vec = vec![vec![0; width]; height];
        for i in (0..len2).rev().step_by(1) {
            let a = num2[i];
            for j in (0..len1).rev().step_by(1) {
                let b = num1[j];
                let row = len2 - 1 - i;
                let cell = len1 - 1 - j + (len2 - i - 1);
                mul_vec[row][cell] = a * b;
            }
        }
        let mut sum_vec = vec![];
        for i in 0..width {
            let mut sum = 0;
            for row in 0..height {
                sum += mul_vec[row][i];
            }
            sum_vec.push(sum);
        }
        if sum_vec.len() <= 1 {
            return sum_vec[0].to_string();
        }
        loop {
            for i in 1..sum_vec.len() {
                let prev_val = sum_vec[i - 1];
                if prev_val / 10 > 0 {
                    sum_vec[i - 1] = prev_val % 10;
                    sum_vec[i] += prev_val / 10;
                }
                if i == sum_vec.len() - 1 {
                    let last = sum_vec[i];
                    if last < 10 {
                        return sum_vec.into_iter().rev().map(|i| i.to_string()).collect();
                    } else {
                        sum_vec[i] = last % 10;
                        sum_vec.push(last / 10);
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let a = 999;
        let b = 678;
        let c = a * b;
        assert_eq!(
            Solution::multiply(a.to_string(), b.to_string()),
            c.to_string()
        );
    }

    #[test]
    fn t2() {
        let a = 2;
        let b = 3;
        let c = a * b;
        assert_eq!(
            Solution::multiply(a.to_string(), b.to_string()),
            c.to_string()
        );
    }

    #[test]
    fn t3() {
        let a = 112;
        let b = 63;
        let c = a * b;
        assert_eq!(
            Solution::multiply(a.to_string(), b.to_string()),
            c.to_string()
        );
    }
    #[test]
    fn t4() {
        let a = 101;
        let b = 3;
        let c = a * b;
        assert_eq!(
            Solution::multiply(a.to_string(), b.to_string()),
            c.to_string()
        );
    }
    #[test]
    fn t5() {
        let a = 12345;
        let b = 0;
        let c = a * b;
        assert_eq!(
            Solution::multiply(a.to_string(), b.to_string()),
            c.to_string()
        );
    }
}
