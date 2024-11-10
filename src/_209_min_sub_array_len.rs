use std::cmp::min;
///
///https://leetcode.cn/problems/minimum-size-subarray-sum/
///
struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        // 滑动窗口起始位置i
        let mut i = 0;
        let mut min_len = i32::MAX as usize;
        let mut exist = false;
        // 滑动窗口终止位置j
        for (j, v) in nums.iter().enumerate() {
            sum += v;
            while sum >= target {
                // 尝试移动滑动窗口起始位置
                let sub = j - i + 1;
                min_len = min(min_len, sub);
                sum -= nums[i];
                i += 1;
                exist = true;
            }
        }
        if exist {
            min_len as i32
        } else {
            // 没有满足条件的子数组
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let len = Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]);
        assert_eq!(len, 2);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn t3() {
        let len = Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(len, 0);
    }
}