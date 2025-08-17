///
/// [540. 有序数组中的单一元素](https://leetcode.cn/problems/single-element-in-a-sorted-array/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mut mid = left + ((right - left) >> 1);
            if mid % 2 == 1 {
                mid -= 1;
            }
            if nums[mid] == nums[mid + 1] {
                // nums[mid]和nums[mid + 1]相等,则说明左边的数字都是双元素的(数组是有序的)
                left = mid + 2;
            } else {
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
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }
}
