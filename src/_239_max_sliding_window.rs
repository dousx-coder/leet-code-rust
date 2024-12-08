///
///
/// 239. 滑动窗口最大值
///
/// https://leetcode.cn/problems/sliding-window-maximum/description/
///
///
///
use std::collections::VecDeque;

struct Solution;
///
/// 单调队列
///
struct CusMonotonicVecDeque {
    que: VecDeque<i32>,
}
impl CusMonotonicVecDeque {
    fn new() -> CusMonotonicVecDeque {
        CusMonotonicVecDeque {
            que: VecDeque::new(),
        }
    }
    fn pop(&mut self, value: i32) {
        let deque = &mut self.que;
        if !deque.is_empty() && value == *deque.front().unwrap() {
            deque.pop_front();
        }
    }

    fn push(&mut self, value: i32) {
        let deque = &mut self.que;
        while !deque.is_empty() && value > *deque.back().unwrap() {
            deque.pop_back();
        }
        deque.push_back(value);
    }
    /// 查询当前队列里的最大值
    fn front(&self) -> i32 {
        let deque = &self.que;
        *deque.front().unwrap()
    }
}
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 最大堆 超时
        let k = k as usize;
        let mut result = Vec::new();
        let mut que = CusMonotonicVecDeque::new();
        for i in 0..k {
            // 先将前k的元素放进队列
            que.push(nums[i]);
        }
        result.push(que.front());
        for i in k..nums.len() {
            // 滑动窗口移除最前面元素
            que.pop(nums[i - k]);
            // 滑动窗口前加入最后面的元素
            que.push(nums[i]);
            // 记录对应的最大值
            result.push(que.front());
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
