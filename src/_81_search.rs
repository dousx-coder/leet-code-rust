///
///[81.  搜索旋转排序数组 II](https://leetcode.cn/problems/search-in-rotated-sorted-array-ii/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                return true;
            }

            // 去重：当左端点等于中间点时，无法判断哪边有序，只能缩小区间
            if nums[left] == nums[mid] {
                left += 1;
                continue;
            }

            // 前半段有序
            if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // 后半段有序
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}
