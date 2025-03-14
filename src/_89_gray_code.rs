/// 89 格雷编码
///
/// https://leetcode.cn/problems/gray-code/description/?envType=problem-list-v2&envId=backtracking
///
struct Solution;
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // 用来表示二进制位，下标0表示最低位 false 表示0
        let mut binary = vec![false; n as usize];
        let mut result = vec![];
        let len = 2u64.pow(n as u32) as usize;
        let mut used = vec![false; len];
        // 减少初始判断
        Self::init(&mut used, &mut binary, &mut result);
        Self::backtracking(&mut used, &mut binary, &mut result);
        result
    }
    fn backtracking(used: &mut Vec<bool>, binary: &mut Vec<bool>, result: &mut Vec<i32>) -> bool {
        if result.len() == used.len() {
            return true;
        }
        for i in result.len()..used.len() {
            let last = result.last().unwrap();
            let mut last = Self::decimal_to_binary(result.last().unwrap());
            for j in 0..last.len() {
                binary[j] = last[j];
            }
            for j in 0..binary.len() {
                let bit = binary[j];
                binary[j] = !bit;
                let ans = Self::binary_to_decimal(binary);
                // 回溯
                binary[j] = bit;
                if !used[ans as usize] {
                    result.push(ans as i32);
                    used[ans as usize] = true;
                    if Self::backtracking(used, binary, result) {
                        return true;
                    }
                    // 回溯
                    used[ans as usize] = false;
                    result.pop();
                }
            }
        }
        false
    }

    fn init(used: &mut Vec<bool>, binary: &mut Vec<bool>, result: &mut Vec<i32>) {
        if result.is_empty() {
            let ans = Self::binary_to_decimal(binary);
            result.push(ans as i32);
            used[ans as usize] = true;
        }
    }

    /// 二进制转十进制
    ///
    /// (下标为0表示最低位)
    fn binary_to_decimal(binary: &Vec<bool>) -> u32 {
        // 使用fold方法来累加每一位的值
        binary
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &bit)| acc + ((if bit { 1 } else { 0 }) << i))
    }
    /// 十进制转二进制数组
    ///
    /// (下标为0表示最低位)
    fn decimal_to_binary(num: &i32) -> Vec<bool> {
        let binary = format!("{:b}", num);
        binary
            .chars()
            .rev()
            .map(|c| if c == '1' { true } else { false })
            .collect::<Vec<bool>>()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
    }
    #[test]
    fn t3() {
        //0 ~ 7
        let ans = Solution::gray_code(3);
        println!("{:?}", ans);
        assert_eq!(ans, vec![0, 1, 3, 2, 6, 7, 5, 4]);
    }
}
