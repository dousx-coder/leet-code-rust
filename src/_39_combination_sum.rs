///
/// 39. 组合总和
///
/// https://leetcode.cn/problems/combination-sum/description/
///
use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut list: Vec<i32> = Vec::new();
        let mut ans: BTreeSet<Vec<i32>> = BTreeSet::new();
        Self::dfs(&candidates, target, &mut list, &mut ans);
        ans.into_iter().collect()
    }
    fn dfs(candidates: &Vec<i32>, target: i32, curr: &mut Vec<i32>, ans: &mut BTreeSet<Vec<i32>>) {
        if target < 0 {
            return;
        }
        if target == 0 {
            let mut vec = curr.clone();
            vec.sort();
            ans.insert(vec);
            return;
        }
        candidates.iter().for_each(|value| {
            let sub = target - value;
            if sub >= 0 {
                curr.push(value.clone());
                Self::dfs(candidates, sub, curr, ans);
                curr.pop();
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let solution = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        assert_eq!((vec![vec![2, 2, 3], vec![7]]), solution);
    }
    #[test]
    fn t2() {
        let solution = Solution::combination_sum(vec![2, 3, 5], 8);
        assert_eq!(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]], solution);
    }
    #[test]
    fn t3() {
        let solution = Solution::combination_sum(vec![2], 1);
        assert!(solution.is_empty())
    }
    #[test]
    fn t4() {
        let solution = Solution::combination_sum(vec![8, 7, 4, 3], 11);
        assert_eq!((vec![vec![3, 4, 4], vec![3, 8], vec![4, 7]]), solution);
    }
}
