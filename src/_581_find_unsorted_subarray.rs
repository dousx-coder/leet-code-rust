/// 581 最短无序子序列
///
/// https://leetcode.cn/problems/shortest-unsorted-continuous-subarray/?envType=problem-list-v2&envId=greedy
struct Solution;
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max = i32::MIN;
        let mut min = i32::MAX;

        let len = nums.len();
        for i in 0..len {
            if nums[i] > max {
                max = nums[i];
            } else if nums[i] < max {
                //  敲定右边界
                right = i;
            }
        }

        for i in (0..len).rev() {
            if nums[i] < min {
                min = nums[i];
            } else if nums[i] > min {
                //  敲定左边界
                left = i;
            }
        }
        if left == right {
            // 原数组是升序
            0
        } else {
            (right - left + 1) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
            5
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 1, 4, 8, 10, 9, 15]),
            7
        );
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    }
    #[test]
    fn t4() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
    }

    #[test]
    fn t5() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 3, 2]), 2);
    }
}
