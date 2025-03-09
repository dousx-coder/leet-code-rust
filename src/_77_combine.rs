/// 77 组合
///
/// https://leetcode.cn/problems/combinations/description/
///
struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut curr = vec![];
        let mut result = vec![];
        Self::backtracking(n, k as usize, 1, &mut result, &mut curr);
        result
    }
    fn backtracking(n: i32, k: usize, start: i32, result: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>) {
        if curr.len() == k {
            result.push(curr.clone());
            return;
        }
        for i in start..=n {
            curr.push(i);
            Self::backtracking(n, k, i + 1, result, curr);
            curr.pop();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let expected = hashset![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ];
        let ans = Solution::combine(4, 2);
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(ans, expected)
    }

    #[test]
    fn t2() {
        let expected = hashset![vec![1]];
        let ans = Solution::combine(1, 1);
        let ans = HashSet::from_iter(ans.into_iter());
        assert_eq!(ans, expected)
    }
}
