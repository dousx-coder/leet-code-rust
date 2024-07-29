///
/// 39. 组合总和
///
/// https://leetcode.cn/problems/combination-sum/description/
///
use std::collections::HashSet;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut can = candidates.clone();
        can.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut list: Vec<i32> = Vec::new();
        let mut set: HashSet<String> = HashSet::new();
        Solution::backtracking(&can, target, &mut result, &mut list, &mut set, target);
        result
    }
    fn backtracking(
        can: &Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
        current_list: &mut Vec<i32>,
        mutable_set: &mut HashSet<String>,
        original_target: i32,
    ) -> bool {
        if target < 0 {
            return false;
        }
        return if target == 0 {
            if !current_list.is_empty() && current_list.iter().sum::<i32>() == original_target {
                current_list.sort();
                let str = current_list
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<String>();
                if mutable_set.insert(str) {
                    let vec = current_list.clone();
                    result.push(vec);
                }
            }
            true
        } else {
            can.iter().for_each(|ca| {
                let sub = target - ca;
                if sub >= 0 {
                    current_list.push(ca.clone());
                    let mut copy = current_list.clone();
                    if !Solution::backtracking(
                        can,
                        sub,
                        result,
                        &mut copy,
                        mutable_set,
                        original_target,
                    ) {
                        current_list.pop();
                    }
                }
            });
            false
        };
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
