///
/// [315. 计算右侧小于当前元素的个数](https://leetcode.cn/problems/count-of-smaller-numbers-after-self/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        Self::binary_search(nums)
    }
    /// 二分检索（超时）
    fn binary_search(nums: Vec<i32>) -> Vec<i32> {
        let mut auxiliary = vec![];
        let mut result = vec![];
        for i in (0..nums.len()).rev() {
            let x = Self::binary(&auxiliary, nums[i]);
            result.insert(0, x as i32);
            auxiliary.insert(x, nums[i]);
        }
        result
    }

    fn binary(auxiliary: &Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = auxiliary.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if auxiliary[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }
}
