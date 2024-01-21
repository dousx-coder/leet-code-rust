use std::collections::HashSet;
/**
 * https://leetcode.cn/problems/first-missing-positive/
 */
struct Solution {}
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut positive_num_set: HashSet<i32> = HashSet::new();
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for it in nums {
            if it >= 0 {
                positive_num_set.insert(it);
                if (it < min) {
                    min = it
                }
                if (it > max) {
                    max = it
                }
            }
        }

        if positive_num_set.len() == 0 {
            return 1;
        }
        for number in 1..max {
            if !positive_num_set.contains(&number) {
                return number;
            }
        }
        return max + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
