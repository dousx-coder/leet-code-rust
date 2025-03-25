///
/// 1079 活字印刷 (排序+子集+去重)
///
/// 排列([a,b]和[b,a])是2种解
///
/// https://leetcode.cn/problems/letter-tile-possibilities/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut chars = tiles.chars().collect::<Vec<char>>();
        chars.sort();
        let mut ans = 0;
        Self::backtracking(
            0,
            &mut ans,
            &mut String::new(),
            &mut vec![false; chars.len()],
            &chars,
        );
        ans
    }
    fn backtracking(
        pos: usize,
        ans: &mut i32,
        path: &mut String,
        used: &mut Vec<bool>,
        chars: &Vec<char>,
    ) {
        if pos == chars.len() {
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
            *ans += 1;
            Self::backtracking(pos + 1, ans, path, used, chars);
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
