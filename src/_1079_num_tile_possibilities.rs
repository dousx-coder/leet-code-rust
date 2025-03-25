///
/// 1079 活字印刷
///
/// https://leetcode.cn/problems/letter-tile-possibilities/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut chars = tiles.chars().collect::<Vec<char>>();
        chars.sort();
        let ans = &mut vec![];
        Self::backtracking(
            0,
            ans,
            &mut String::new(),
            &mut vec![false; chars.len()],
            &chars,
        );
        ans.len() as i32
    }
    fn backtracking(
        size: usize,
        ans: &mut Vec<String>,
        path: &mut String,
        used: &mut Vec<bool>,
        chars: &Vec<char>,
    ) {
        if size == chars.len() {
            return;
        }
        for i in 0..chars.len() {
            if used[i] {
                continue;
            }
            if i > 0 && chars[i] == chars[i - 1] && !used[i - 1] {
                continue;
            }
            used[i] = true;
            path.push(chars[i]);
            ans.push(path.clone());
            Self::backtracking(size + 1, ans, path, used, chars);
            path.pop();
            used[i] = false;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
    }
}
