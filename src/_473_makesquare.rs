/// 473  火柴拼正方形
///
/// https://leetcode.cn/problems/matchsticks-to-square/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let target_side_len = sum / 4;
        let mut matchsticks = matchsticks;

        // 从大到小排序
        matchsticks.sort_unstable_by(|a, b| b.cmp(a));

        let len = matchsticks.len();
        let mut used = vec![false; len];

        Self::backtrack(&matchsticks, &mut used, target_side_len, 0, 0, 0)
    }

    fn backtrack(
        matchsticks: &Vec<i32>,
        used: &mut Vec<bool>,
        target: i32,
        index: usize,
        sides_total: i32,
        curr_side_len: i32,
    ) -> bool {
        if sides_total == 4 {
            return true;
        }

        if curr_side_len == target {
            return Self::backtrack(matchsticks, used, target, 0, sides_total + 1, 0);
        }

        for i in index..matchsticks.len() {
            if used[i] {
                continue;
            }

            if curr_side_len + matchsticks[i] > target {
                continue;
            }

            // 跳过重复火柴
            if i > 0 && matchsticks[i] == matchsticks[i - 1] && !used[i - 1] {
                continue;
            }

            used[i] = true;

            if Self::backtrack(
                matchsticks,
                used,
                target,
                i + 1,
                sides_total,
                curr_side_len + matchsticks[i],
            ) {
                return true;
            }
            // 回溯
            used[i] = false;

            // 如果当前边总和为0，且当前火柴无法加入，说明无解
            if curr_side_len == 0 {
                return false;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3]),
            true
        );
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::makesquare(vec![5, 5, 5, 5, 16, 4, 4, 4, 4, 4, 3, 3, 3, 3, 4]),
            false
        );
    }
}
