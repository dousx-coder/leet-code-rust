///
/// [393. UTF-8编码验证](https://leetcode.cn/problems/utf-8-validation/description/)
///
struct Solution;
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let bins: Vec<_> = data.iter().map(|x| format!("{:08b}", x)).collect();
        let mut i = 0;
        let len = bins.len();
        while i < len {
            let bin = &bins[i];
            if bin.starts_with("0") {
                // 单字节字符
                i += 1;
            } else if bin.starts_with("110") {
                // 两字节字符
                if let Some(true) = Self::verify(&bins, i, len, "110") {
                    i += 2;
                } else {
                    return false;
                }
            } else if bin.starts_with("1110") {
                // 三字节字符
                if let Some(true) = Self::verify(&bins, i, len, "1110") {
                    i += 3;
                } else {
                    return false;
                }
            } else if bin.starts_with("11110") {
                // 四字节字符
                if let Some(true) = Self::verify(&bins, i, len, "11110") {
                    i += 4;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn verify(bins: &Vec<String>, index: usize, len: usize, start: &str) -> Option<bool> {
        let bty_len = start.len() - 1;
        if index + bty_len <= len {
            for k in (index + 1)..(index + bty_len) {
                if !bins[k].starts_with("10") {
                    return Some(false);
                }
            }
        } else {
            return Some(false);
        }
        Some(true)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::valid_utf8(vec![230, 136, 145]), true);
    }
}
