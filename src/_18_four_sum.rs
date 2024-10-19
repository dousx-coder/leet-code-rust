use std::collections::HashSet;
///
/// https://leetcode.cn/problems/4sum/description/
///
struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
                            result.push(vec![sort_nums[a], sort_nums[b], sort_nums[c], sort_nums[d]]);
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
}