///
/// [28. 找出字符串中第一个匹配项的下标](https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/description/)
///
struct Solution;
impl Solution {
    /// KMP算法实现
    fn prefix_table(pattern: &Vec<u8>) -> Vec<i32> {
        // 最长相等前后缀(单字符不存在最长相等前后缀)
        // aab
        // 前缀:a,aa (不包含最后一个字符)
        // 后缀:b,ab (不包含第一个字符)
        // 不存在最长相等的前后缀

        // aaba
        // 前缀: a,aa,aab
        // 后缀: a,ba,aba
        // 相同的前后缀为a，即最长相等前后缀是a
        let n = pattern.len();
        let mut prefix_table: Vec<i32> = vec![0; n];
        let mut i: usize = 1;
        let mut len: i32 = 0;
        while i < n {
            if pattern[i] == pattern[len as usize] {
                len += 1;
                prefix_table[i] = len;
                i += 1;
            } else {
                if len > 0 {
                    let index = (len - 1) as usize;
                    len = prefix_table[index];
                } else {
                    prefix_table[i] = len;
                    i += 1;
                }
            }
        }
        // ABABCABAA的最长公共前缀数组，第i位表示[0..i]字符串对应的最长公共前后缀的长度
        //  [0, 0, 1, 2, 0, 1, 2, 3, 1] 原最长公共前后缀长度
        // [-1, 0, 0, 1, 2, 0, 1, 2, 3] 向右移动一位

        // 将prefix_table整体向右移动一位
        let mut index = n - 1;
        while index > 0 {
            prefix_table[index] = prefix_table[index - 1];
            index -= 1;
        }
        prefix_table[0] = -1;
        prefix_table
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let text = haystack.as_bytes().to_vec();
        let pattern = needle.as_bytes().to_vec();
        let m = haystack.len();
        let n = needle.len();
        let prefix_table = Self::prefix_table(&pattern);
        let mut i = 0;
        let mut j = 0;
        // 说明(KMP核心思想是在遇到不匹配的字符串时,最大化的利用已经匹配过的字符串)
        // haystack：ABFNABFNABCABAACCABDABABCABAACC
        // needle：  ABFNABCABAA
        // 第一次出现i和j指定的字符不相等时(i=6,j=6)
        // h1： ABFNABF
        // n1： ABFNABC
        // 去掉h1和n1的最后一位字符,得到2个字符串,h2(ABFNAB) n2(ABFNAB)
        //  需要比较h2的后缀 和n2的前缀,找到一个相同且长度最长的字串,此时便能重置j
        while i < m {
            if j == n - 1 && text[i] == pattern[j] {
                return (i - j) as i32;
            }
            if text[i] == pattern[j] {
                i += 1;
                j += 1;
            } else {
                // 字符不匹配
                j = if prefix_table[j] < 0 {
                    // j对应的前缀长度小于0，则需要重新从头匹配
                    i += 1;
                    0
                } else {
                    // 利用最长的公共(相同)前缀
                    prefix_table[j] as usize
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ans = Solution::str_str(
            String::from("ABFNABFNABCABAACCABDABABCABAACC"),
            String::from("ABFNABCABAA"),
        );
        assert_eq!(ans, 4);
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solution::str_str(String::from("sadbutsad"), String::from("sad")),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::str_str(String::from("leetcode"), String::from("leeto")),
            -1
        );
    }
}
