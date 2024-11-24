use std::collections::HashMap;

///
/// https://leetcode.cn/problems/4sum-ii/
struct Solution;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for a in nums1 {
            for b in &nums2 {
                let key = a + b;
                let v = map.get(&key).or(Some(&0)).unwrap() + 1;
                map.insert(key, v);
            }
        }
        let mut ans = 0;
        for c in nums3 {
            for d in &nums4 {
                let key = 0 - c - d;
                ans += map.get(&key).or(Some(&0)).unwrap();
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]);
        assert_eq!(ans, 2);
    }

    #[test]
    fn t2() {
        let ans = Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]);
        assert_eq!(ans, 1);
    }

    #[test]
    fn t3() {
        let ans = Solution::four_sum_count(vec![-1, -1], vec![-1, 1], vec![-1, 1], vec![1, -1]);
        assert_eq!(ans, 6);
    }
}
