/// 153 寻找旋转数组中的最小值
///
/// https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/?envType=problem-list-v2&envId=binary-search
struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            // 防止溢出
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                // [mid+1..right]是旋转数组
                // 旋转点在右边
                left = mid + 1;
                continue;
            }
            if nums[mid] < nums[right] {
                // [left..mid]是旋转数组
                // 旋转点在左边
                right = mid;
                continue;
            }
            right -= 1;
        }
        nums[left]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }
}
