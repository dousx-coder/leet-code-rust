use std::collections::HashMap;
///
/// [532. 数组中的k-diff数对](https://leetcode.cn/problems/k-diff-pairs-in-an-array/)
///
struct Solution;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        // Self::hash(nums, k)
        Self::binary_search(nums, k)
    }

    /// 哈希表解法
    fn hash(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        if k < 0 {
            return 0;
        }
        let mut map = HashMap::new();
        // 统计数组中每个数字出现的次数
        nums.iter().for_each(|num| {
            let c = match map.get(num) {
                Some(v) => *v,
                None => 0,
            } + 1;
            map.insert(num, c);
            // match匹配比entry更快
            // *map.entry(num).or_insert(0) += 1;
        });
        for num in map.keys() {
            if k == 0 {
                // k==0,找相同数字
                if *map.get(num).unwrap() > 1 {
                    count += 1;
                }
            } else {
                if map.contains_key(&(*num + k)) {
                    count += 1;
                }
            }
        }
        count
    }

    /// 二分
    fn binary_search(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut count = 0;
        let len = nums.len();
        let mut i = 0;
        while i < len - 1 {
            let t = nums[i] + k;
            let mut left = i + 1;
            let mut right = len - 1;
            while left < right {
                let mid = left + right + 1 >> 1;
                if nums[mid] <= t {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            count += if nums[left] == t { 1 } else { 0 };
            let mut j = i + 1;
            while j < len && nums[j] == nums[i] {
                j += 1;
            }
            i = j;
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::hash(vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3), 2);
    }

    #[test]
    fn t5() {
        assert_eq!(
            Solution::binary_search(vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3),
            2
        );
    }
}
