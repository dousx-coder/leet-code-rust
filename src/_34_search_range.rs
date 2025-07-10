///
/// [34. 在排序数组中查找元素的第一个和最后一个位置](https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/)
///
struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        if len == 0 {
            return vec![-1, -1];
        }
        let mut left = 0;
        let mut right = len - 1;
        let mut last = -1;
        let mut first = -1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                // 中间值
                left = mid + 1;
                continue;
            }
            if nums[mid] > target {
                if mid < 1 {
                    // 类型越界
                    return vec![-1, -1];
                }
                right = mid - 1;
                continue;
            }
            if nums[mid] == target {
                for i in mid..=right {
                    if nums[i] == target {
                        last = i as i32;
                    }
                }
                for i in (left..=mid).rev() {
                    if nums[i] == target {
                        first = i as i32;
                    }
                }
                return vec![first, last];
            }
        }
        vec![-1, -1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        assert_eq!(Solution::search_range(nums, 8), vec![3, 4])
    }

    #[test]
    fn t2() {
        let nums = vec![1];
        assert_eq!(Solution::search_range(nums, 1), vec![0, 0])
    }

    #[test]
    fn t3() {
        let nums = vec![1];
        assert_eq!(Solution::search_range(nums, 0), vec![-1, -1])
    }
}
