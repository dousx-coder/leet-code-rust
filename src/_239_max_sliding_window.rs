use std::collections::BinaryHeap;
///
///
/// 239. 滑动窗口最大值
///
/// https://leetcode.cn/problems/sliding-window-maximum/description/
///
struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 最大堆 超时
        let k = k as usize;
        let mut result = vec![];
        let mut i = 0;
        loop {
            let end = i + k;
            if end > nums.len() {
                break;
            }
            let start = i;
            let mut heap = BinaryHeap::new();
            for i in start..end {
                heap.push(nums[i]);
            }
            result.push(heap.pop().unwrap());
            i += 1;
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(ans, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn t2() {
        let ans = Solution::max_sliding_window(vec![1], 1);
        assert_eq!(ans, vec![1]);
    }
}
