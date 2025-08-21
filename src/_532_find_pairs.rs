use std::collections::HashMap;
///
/// [532. 数组中的k-diff数对](https://leetcode.cn/problems/k-diff-pairs-in-an-array/)
///
struct Solution;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        if k < 0 {
            return count;
        }
        let mut map = HashMap::new();
        // 统计数组中每个数字出现的次数
        nums.iter().for_each(|num| {
            *map.entry(num).or_insert(0) += 1;
        });
        for i in map.keys() {
            if k == 0 {
                // k==0,找相同数字
                if *map.get(i).unwrap() > 1 {
                    count += 1;
                }
            } else {
                if map.contains_key(&(*i + k)) {
                    count += 1;
                }
            }
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
        assert_eq!(
            Solution::find_pairs(vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3),
            2
        );
    }
}
