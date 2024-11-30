///
/// https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
///
/// KMP算法实现
struct Solution {}

impl Solution {
    fn prefix_table(pattern: &Vec<u8>) -> Vec<i32> {
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
        // 将prefix_table整体向右移动一位
        // [0,1,0,2,0]===> [-1,0,1,0,2]
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
        let prefix_table = Solution::prefix_table(&pattern);
        let mut i = 0;
        let mut j = 0;
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
        Solution::str_str(
            String::from("ABABABCABAACCABDABABCABAACC"),
            String::from("ABABCABAA"),
        );
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
