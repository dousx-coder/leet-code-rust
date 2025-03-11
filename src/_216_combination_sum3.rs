///
/// 216 组合数字 Ⅲ
///
/// https://leetcode.cn/problems/combination-sum-iii/
struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtracking(k, n, 1, &mut result, &mut vec![]);
        result
    }
    fn backtracking(k: i32, n: i32, start: i32, result: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>) {
        if n == 0 && k == 0 {
            result.push(curr.clone());
            return;
        }
        if n < 0 || k < 0 {
            return;
        }
        for i in start..=9 {
            curr.push(i);
            Self::backtracking(k - 1, n - i, i + 1, result, curr);
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
        let ans = Solution::combination_sum3(3, 9);
        let expect = hashset! {vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]};
        assert_eq!(HashSet::from_iter(ans.into_iter()), expect);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    }
}
