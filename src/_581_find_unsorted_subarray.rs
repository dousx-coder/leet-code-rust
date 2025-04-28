/// 581 最短无序子序列
///
/// https://leetcode.cn/problems/shortest-unsorted-continuous-subarray/?envType=problem-list-v2&envId=greedy
struct Solution;
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return 0;
        }

        let mut left = 0;
        let mut right = 0;
        let mut max = nums[0];
        let mut min = nums[len - 1];

        // 从左到右确定右边界
        for i in 1..len {
            if nums[i] < max {
                right = i;
            } else {
                max = nums[i];
            }
        }

        // 从右到左确定左边界
        for i in (0..len - 1).rev() {
            if nums[i] > min {
                left = i;
            } else {
                min = nums[i];
            }
        }

        if left == right {
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
