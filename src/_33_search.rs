/// 33 搜索旋转排序数组
///
/// https://leetcode.cn/problems/search-in-rotated-sorted-array/
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        // 对数组进行二分，其中一定有一个子区间是有序的，另一个部分有序
        // 1.如果target在这个有序区间内：直接二分
        // 2.如果target在另一个区间内：二分后仍得到一个有序和部分有序区间，
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[left] <= nums[mid] {
                // [left, mid]有序
                if target >= nums[left] && target <= nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                // [mid + 1, right]有序
                if target > nums[mid] && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0);
        assert_eq!(ans, 4);
    }
    #[test]
    fn t2() {
        let ans = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3);
        assert_eq!(ans, -1);
    }
    #[test]
    fn t3() {
        let ans = Solution::search(vec![1], 0);
        assert_eq!(ans, -1);
    }

    #[test]
    fn t4() {
        let ans = Solution::search(vec![5, 1, 3], 0);
        assert_eq!(ans, -1);
    }
}
