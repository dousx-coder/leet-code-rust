///
/// [393. UTF-8编码验证](https://leetcode.cn/problems/utf-8-validation/description/)
///
struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut i = 0;
        while i < data.len() {
            let byte = data[i];
            match Self::get_utf8_header_info(byte) {
                Some(1) => i += 1,
                Some(n) => {
                    if Self::is_valid_sequence(&data, i, n) {
                        i += n;
                    } else {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }

    /// 获取 UTF-8 头字节的信息
    /// 返回 Some(n) 表示该字符由 n 个字节组成
    /// 返回 None 表示无效的头字节
    fn get_utf8_header_info(byte: i32) -> Option<usize> {
        // 检查是否为单字节字符 (0xxxxxxx)
        if byte & 0b10000000 == 0 {
            return Some(1);
        }

        // 检查是否为多字节字符的头字节
        let mut count = 0;
        let mut mask = 0b10000000;

        // 计算前导1的个数
        while byte & mask != 0 {
            count += 1;
            mask >>= 1;

            // UTF-8 最多支持 4 字节
            if count > 4 {
                return None;
            }
        }

        // 有效的多字节字符必须有 2-4 个字节
        if count >= 2 {
            Some(count)
        } else {
            None
        }
    }

    /// 验证从 index 开始的 n 字节 UTF-8 序列是否有效
    fn is_valid_sequence(data: &[i32], index: usize, n: usize) -> bool {
        // 检查是否有足够的字节
        if index + n > data.len() {
            return false;
        }

        // 检查后续字节是否都是 10xxxxxx 格式
        for i in (index + 1)..(index + n) {
            if data[i] & 0b11000000 != 0b10000000 {
                return false;
            }
        }

        true
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

    #[test]
    fn test_single_byte() {
        assert_eq!(Solution::valid_utf8(vec![0, 127]), true);
    }

    #[test]
    fn test_invalid_header() {
        assert_eq!(Solution::valid_utf8(vec![128]), false);
    }
}
