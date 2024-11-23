use std::collections::HashSet;
use std::hash::Hash;

///
/// https://leetcode.cn/problems/intersection-of-two-arrays/
///
struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set: HashSet<i32> = HashSet::from_iter(nums1);
        let mut ans = HashSet::new();
        for x in nums2 {
            if set.contains(&x) {
                ans.insert(x);
            }
        }
        ans.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        assert_eq!(ans, vec![2]);
    }
    #[test]
    fn t2() {
        let ans = Solution::intersection(vec![1, 2, 3, 1], vec![3, 2]);
        assert_eq!(ans, vec![2, 3]);
    }
}
