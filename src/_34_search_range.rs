struct Solution;
///
/// 34 在排序数组中查找元素的第一个和最后一个位置
///
/// https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        if len == 0 {
            return vec![-1, -1];
        }

        let first = Self::find_first(&nums, target);
        if first == -1 {
            return vec![-1, -1];
        }
        let last = Self::find_last(&nums, target);

        vec![first, last]
    }

    fn find_first(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                if mid == 0 || nums[mid as usize - 1] != target {
                    return mid;
                }
                right = mid - 1;
            }
        }
        -1
    }

    fn find_last(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                if mid == nums.len() as i32 - 1 || nums[mid as usize + 1] != target {
                    return mid;
                }
                left = mid + 1;
            }
        }
        -1
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
