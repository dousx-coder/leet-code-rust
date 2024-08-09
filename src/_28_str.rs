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
        let mut index = n - 1;
        while index > 0 {
            prefix_table[index] = prefix_table[index - 1];
            index -= 1;
        }
        prefix_table[0] = -1;
        return prefix_table;
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
                if prefix_table[j] < 0 {
                    i += 1;
                    j = 0;
                } else {
                    j = prefix_table[j] as usize;
                }
            }
        }
        return -1;
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
