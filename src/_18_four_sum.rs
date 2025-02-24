use std::collections::HashSet;
///
/// https://leetcode.cn/problems/4sum/description/
///
struct Solution;

impl Solution {
    /// 循环解法
    fn loop_solution(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let len = nums.len();
        if len < 4 {
            return vec![];
        }
        let target = target as i64;
        let mut result = HashSet::new();
        nums.sort();
        // 提前进行类型转换，降低内存占用
        let nums = nums.iter().map(|x| *x as i64).collect::<Vec<i64>>();
        // len -3 得到的结果可能不是usize
        for a in 0..len - 3 {
            if target > 0 && nums[a] > target {
                // 剪枝
                break;
            }
            if a > 0 && nums[a] == nums[a - 1] {
                // 去重
                continue;
            }
            if nums[a] + nums[a + 1] + nums[a + 2] + nums[a + 3] > target {
                break;
            }
            if (nums[a] + nums[len - 1] + nums[len - 2] + nums[len - 3]) < target {
                continue;
            }
            for b in a + 1..len - 2 {
                if b > a + 1 && nums[b] == nums[b - 1] {
                    // 去重
                    continue;
                }
                if target > 0 && nums[a] + nums[b] > target {
                    // 剪枝
                    break;
                }
                if nums[a] + nums[b] + nums[b + 1] + nums[b + 2] > target {
                    break;
                }
                if (nums[a] + nums[b] + nums[len - 1] + nums[len - 2]) < target {
                    continue;
                }
                // 双指针 两数之和
                let mut left = b + 1;
                let mut right = len - 1;
                while left < right {
                    let sum = nums[a] + nums[b] + nums[left] + nums[right];
                    if sum < target {
                        left += 1;
                        continue;
                    }
                    if sum > target {
                        right -= 1;
                        continue;
                    }
                    result.insert(vec![
                        nums[a] as i32,
                        nums[b] as i32,
                        nums[left] as i32,
                        nums[right] as i32,
                    ]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }
        result.into_iter().collect()
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
                let four_sum = (a + b + c + d) as i64;
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
        Self::loop_solution(nums, target)
        // Self::recursion_solution(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(nums: Vec<i32>, target: i32, expect: Vec<Vec<i32>>) {
        let solution = Solution::four_sum(nums, target);
        let ans: HashSet<Vec<i32>> = HashSet::from_iter(solution.into_iter());
        assert_eq!(ans, HashSet::from_iter(expect.into_iter()));
    }

    #[test]
    fn t1() {
        testing(
            vec![1, 0, -1, 0, -2, 2],
            0,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
        );
    }
    #[test]
    fn t2() {
        testing(vec![0], 0, vec![]);
    }
    #[test]
    fn t3() {
        testing(
            vec![
                -500, -498, -414, -406, -404, -397, -395, -389, -349, -274, -253, -249, -238, -222,
                -215, -201, -171, -159, -157, -156, -156, -110, -89, -80, -76, -75, -70, -52, -9,
                -2, 1, 8, 40, 52, 58, 60, 98, 116, 143, 148, 151, 165, 165, 219, 236, 244, 259,
                285, 292, 318, 319, 331, 337, 347, 360, 363, 365, 430, 443, 444, 470, 472,
            ],
            6111,
            vec![],
        );
    }

    #[test]
    fn t4() {
        testing(
            vec![0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000],
            1000000000,
            vec![vec![0, 0, 0, 1000000000]],
        );
    }

    #[test]
    fn t5() {
        testing(
            vec![-5, 5, 4, -3, 0, 0, 4, -2],
            4,
            vec![vec![-3, -2, 4, 5], vec![-5, 0, 4, 5]],
        );
    }
}
