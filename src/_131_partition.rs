/// 131 切割回文串
///
/// https://leetcode.cn/problems/palindrome-partitioning/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars = s.chars().collect::<Vec<char>>();
        let mut ans = vec![];
        Self::backtracking(0, &chars, &mut ans, &mut vec![]);
        ans
    }
    fn backtracking(
        start: usize,
        chars: &Vec<char>,
        ans: &mut Vec<Vec<String>>,
        path: &mut Vec<(usize, usize)>,
    ) {
        let len = chars.len();
        if start == len {
            ans.push(
                path.iter()
                    .map(|(i, j)| chars[*i..=*j].iter().collect::<String>())
                    .collect::<Vec<String>>(),
            );
        }
        for end in start..len {
            let x = &chars[start..=end];
            if Self::palindromic(x) {
                path.push((start, end));
                Self::backtracking(end + 1, chars, ans, path);
                path.pop();
            } else {
                // 这里不能break，start..=end 不是回文数，start..=end+1 可能就是回文数
                // break;
            }
        }
    }

    /// true 回文
    fn palindromic(path: &[char]) -> bool {
        if path.len() == 1 {
            return true;
        }
        for i in 0..path.len() / 2 {
            if path[i] != path[path.len() - 1 - i] {
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
    fn test_1() {
        assert_eq!(
            Solution::partition("aab".to_string()),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::partition("a".to_string()), vec![vec!["a"]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::partition("efe".to_string()),
            vec![vec!["e", "f", "e"], vec!["efe"]]
        );
    }
}
