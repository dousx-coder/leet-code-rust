///
/// [154. 寻找旋转排序数组中的最小值 Ⅱ](https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array-ii/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    /// 已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到输入数组。例如，原数组 nums = [0,1,4,4,5,6,7] 在变化后可能得到：
    /// - 若旋转 4 次，则可以得到 [4,5,6,7,0,1,4]
    /// - 若旋转 7 次，则可以得到 [0,1,4,4,5,6,7]
    ///
    ///
    /// [nums]可能存在 重复 元素值的数组
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] == nums[right] {
                // 缩小右边界
                right -= 1;
            } else if nums[mid] > nums[right] {
                left = mid + 1;
            } else if nums[mid] < nums[right] {
                right = mid;
            }
        }
        nums[left]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::find_min(vec![1, 3, 3]), 1);
    }
}
