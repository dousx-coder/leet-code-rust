///
/// 6 Z字形变换
///
/// https://leetcode.cn/problems/zigzag-conversion/description/
///
struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let mut rows: Vec<String> = vec!["".to_string(); num_rows as usize];
        let mut index = 0;
        let mut flag = -1;
        for c in s.chars().into_iter() {
            if index < 0 {
                panic!("invalid index");
            }
            rows[index as usize].push(c);
            if index == 0 || index == (num_rows - 1) {
                flag = -flag;
            }
            index += flag;
        }
        rows.join("")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::convert(String::from("PAYPALISHIRING"), 3);
        assert_eq!(ans, String::from("PAHNAPLSIIGYIR"));
    }

    #[test]
    fn t2() {
        let ans = Solution::convert(String::from("PAYPALISHIRING"), 4);
        assert_eq!(ans, String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn t3() {
        let ans = Solution::convert(String::from("A"), 1);
        assert_eq!(ans, String::from("A"));
    }
}
