///
/// [390. 消除游戏](https://leetcode.cn/problems/elimination-game/?envType=problem-list-v2&envId=recursion)
///
struct Solution;
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        2 * (n / 2 + 1 - Solution::last_remaining(n / 2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_remaining() {
        assert_eq!(Solution::last_remaining(9), 6);
    }
}
