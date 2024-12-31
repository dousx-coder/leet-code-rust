use std::collections::HashSet;

///
/// 40 组合总和Ⅱ
///
/// https://leetcode.cn/problems/combination-sum-ii/description/
///
struct Solution;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() {
            return vec![];
        }
        let mut candidates = candidates;
        candidates.sort();
        let mut current: Vec<i32> = Vec::new();
        let mut ans: HashSet<Vec<i32>> = HashSet::new();
        Self::dfs(&candidates, target, &mut current, &mut ans, target, 0);
        ans.into_iter().collect()
    }
    fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        curr: &mut Vec<i32>,
        ans: &mut HashSet<Vec<i32>>,
        original: i32,
        begin: usize,
    ) {
        if target == 0 {
            ans.insert(curr.clone());
        }
        let len = candidates.len();
        for index in begin..len {
            let value = candidates[index];
            let key = index as i32;
            let sub = target - value;
            if sub < 0 {
                break;
            };
            if index < begin && value == candidates[index - 1] {
                continue;
            }
            curr.push(value);
            Self::dfs(candidates, sub, curr, ans, original, index + 1);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let ans = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        assert_eq!(
            hashset![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            HashSet::from_iter(ans)
        );
    }

    #[test]
    fn t2() {
        let ans = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
        assert_eq!((hashset![vec![1, 2, 2], vec![5]]), HashSet::from_iter(ans));
    }
    #[test]
    fn t3() {
        let vec = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let sum = vec.iter().sum::<i32>();
        let ans = Solution::combination_sum2(vec, 27);
        assert!(ans.is_empty())
    }
}
