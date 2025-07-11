///
///[784. 字母大小写全排列](https://leetcode.cn/problems/letter-case-permutation/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let s = s.chars().collect::<Vec<char>>();
        let mut ans = vec![];
        Self::backtracking(0, &mut ans, &mut String::new(), &s);
        ans
    }
    fn backtracking(pos: usize, ans: &mut Vec<String>, path: &mut String, s: &Vec<char>) {
        if pos == s.len() {
            ans.push(path.clone());
            return;
        }

        let x = s[pos];
        if x <= '9' {
            path.push(x);
            Self::backtracking(pos + 1, ans, path, s);
            path.pop();
            return;
        }

        path.push(x);
        Self::backtracking(pos + 1, ans, path, s);
        path.pop();

        let p = if x >= 'a' {
            x.to_ascii_uppercase()
        } else {
            x.to_ascii_lowercase()
        };
        path.push(p);
        Self::backtracking(pos + 1, ans, path, s);
        path.pop();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let vec = Solution::letter_case_permutation("a1b2".to_string());
        let set = hashset! {
            "a1b2".to_string(),
            "a1B2".to_string(),
            "A1b2".to_string(),
            "A1B2".to_string()
        };
        assert_eq!(vec.len(), set.len());
        assert_eq!(HashSet::from_iter(vec.into_iter()), set);
    }
    #[test]
    fn t2() {
        let vec = Solution::letter_case_permutation("3z4".to_string());
        let set = hashset! {
            "3z4".to_string(),
            "3Z4".to_string(),
        };
        assert_eq!(vec.len(), set.len());
        assert_eq!(HashSet::from_iter(vec.into_iter()), set);
    }
}
