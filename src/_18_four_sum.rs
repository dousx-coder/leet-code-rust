use std::collections::{HashMap, HashSet};
///
/// https://leetcode.cn/problems/4sum/description/
///
struct Solution;

impl Solution {
    /// 循环解法
    fn loop_solution(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sort_nums = nums.clone();
        let len = nums.len();
        let mut result = Vec::new();
        if len < 4 {
            return result;
        }
        sort_nums.sort();
        let mut first = HashSet::new();
        let tar_64 = target as i64;
        // len -3 得到的结果可能不是usize
        for a in 0..len - 3 {
            if !first.insert(sort_nums[a]) {
                continue;
            }
            let mut second = HashSet::new();
            for b in a + 1..len - 2 {
                if !second.insert(sort_nums[b]) {
                    continue;
                }
                let mut third = HashSet::new();
                for c in b + 1..len - 1 {
                    if !third.insert(sort_nums[c]) {
                        continue;
                    }
                    let three_sum = sort_nums[a] as i64 + sort_nums[b] as i64 + sort_nums[c] as i64;
                    for d in c + 1..len {
                        let four_sum = three_sum + sort_nums[d] as i64;
                        if four_sum == tar_64 {
                            result.push(vec![
                                sort_nums[a],
                                sort_nums[b],
                                sort_nums[c],
                                sort_nums[d],
                            ]);
                            break;
                        }
                        if four_sum > tar_64 {
                            break;
                        }
                    }
                }
            }
        }
        result
    }
    fn recursion(
        dept: usize,
        start: usize,
        index_vec: &mut Vec<usize>,
        sort_nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        target: i64,
    ) {
        let mut use_nums = HashSet::new();
        for i in start..sort_nums.len() - dept {
            if dept == 0 {
                let a = sort_nums[index_vec[3]];
                let b = sort_nums[index_vec[2]];
                let c = sort_nums[index_vec[1]];
                let d = sort_nums[i];
                let four_sum = a as i64 + b as i64 + c as i64 + d as i64;
                if four_sum == target {
                    result.push(vec![a, b, c, d]);
                    return;
                };
                if four_sum > target {
                    return;
                }
            } else {
                if use_nums.insert(sort_nums[i]) {
                    index_vec[dept] = i;
                    Self::recursion(dept - 1, i + 1, index_vec, sort_nums, result, target);
                }
            }
        }
    }
    fn recursion_solution(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sort_nums = nums.clone();
        let len = nums.len();
        let mut result = Vec::new();
        if len < 4 {
            return result;
        }
        sort_nums.sort();
        let mut index_vec = vec![0; 4];
        Self::recursion(3, 0, &mut index_vec, &sort_nums, &mut result, target as i64);
        result
    }

    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // Self::loop_solution(nums, target)
        Self::recursion_solution(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let result = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        println!("{:?}", result);
    }
    #[test]
    fn t2() {
        let result = Solution::four_sum(vec![0], 0);
        println!("{:?}", result);
    }
    #[test]
    fn t3() {
        let result = Solution::four_sum(
            vec![
                -500, -498, -414, -406, -404, -397, -395, -389, -349, -274, -253, -249, -238, -222,
                -215, -201, -171, -159, -157, -156, -156, -110, -89, -80, -76, -75, -70, -52, -9,
                -2, 1, 8, 40, 52, 58, 60, 98, 116, 143, 148, 151, 165, 165, 219, 236, 244, 259,
                285, 292, 318, 319, 331, 337, 347, 360, 363, 365, 430, 443, 444, 470, 472,
            ],
            6111,
        );
        println!("{:?}", result);
        assert!(result.is_empty());
    }
}
