///
/// https://leetcode.cn/problems/squares-of-a-sorted-array/description/
///
struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut left = 0;
        let mut right = len - 1;
        let mut result = vec![-1; len];
        let mut write_index = len - 1;
        while left <= right {
            if nums[left].abs() > nums[right].abs() {
                result[write_index] = nums[left].pow(2);
                left += 1;
            } else {
                result[write_index] = nums[right].pow(2);
                if right <= 0 {
                    break;
                }
                right -= 1;
            }
            if write_index == 0 {
                break;
            }
            write_index -= 1;
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let vec = Solution::sorted_squares(vec![-4, -1, 0, 3, 10]);
        assert_eq!(vec, vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn t2() {
        let vec = Solution::sorted_squares(vec![-7, -3, 2, 3, 11]);
        assert_eq!(vec, vec![4, 9, 9, 49, 121]);
    }
    #[test]
    fn t3() {
        let vec = Solution::sorted_squares(vec![1]);
        assert_eq!(vec, vec![1]);
    }
}