use itertools::Itertools;

///
/// 1238 循环码排列
///
/// https://leetcode.cn/problems/circular-permutation-in-binary-representation/description/
struct Solution;
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut bs = format!("{:b}", start);
        while bs.len() < n as usize {
            bs.insert(0, '0');
        }
        let mut binary = bs.chars().map(|c| c == '1').collect_vec();
        let used_len = 2u64.pow(n as u32) as usize;
        let used = &mut vec![false; used_len];
        used[start as usize] = true;
        let mut ans = vec![start];
        Self::backtracking(&mut binary, &mut ans, used);
        ans
    }

    ///
    /// [`binary`] true表示1，false表示0
    ///
    fn backtracking(binary: &mut Vec<bool>, ans: &mut Vec<i32>, used: &mut Vec<bool>) -> bool {
        if ans.len() == used.len() {
            return Self::verify(ans);
        }
        for _ in ans.len()..used.len() {
            for i in 0..binary.len() {
                let bit = binary[i];
                binary[i] = !bit;
                let mut bs = binary
                    .iter()
                    .map(|b| if *b { '1' } else { '0' })
                    .collect::<String>();
                // 转换为十进制
                let num = i32::from_str_radix(bs.as_str(), 2).unwrap();
                let ui = num as usize;
                if used[ui] {
                    continue;
                }
                used[ui] = true;
                ans.push(num);
                if Self::backtracking(binary, ans, used) {
                    return true;
                }
                // binary回溯
                binary[i] = bit;
                // used回溯
                used[ui] = false;
                ans.pop();
            }
        }
        false
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
