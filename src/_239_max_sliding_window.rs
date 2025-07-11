use std::collections::VecDeque;

///
/// [239. 滑动窗口最大值](https://leetcode.cn/problems/sliding-window-maximum/description/)
///
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
    ///[`value`] 上个区间的起始值
    ///
    /// 当前队首元素等于上个元素区间起始值则弹出队首值(避免影响下次查找最大值)
    fn pop(&mut self, value: i32) {
        let deque = &mut self.que;
        if !deque.is_empty() && value == *deque.front().unwrap() {
            deque.pop_front();
        }
    }
    /// 尝试将元素添加到双向队列队尾
    ///
    /// 如果push的元素value大于入口元素的数值，那么就将队列入口的元素弹出，直到push元素的数值小于等于队列入口元素的数值为止
    fn push(&mut self, value: i32) {
        let deque = &mut self.que;
        while !deque.is_empty() && value > *deque.back().unwrap() {
            deque.pop_back();
        }
        deque.push_back(value);
    }
    /// 获取队首元素值
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
            // 尝试移除双向队列队首元素(即上次滑动窗口的起始值)
            que.pop(nums[i - k]);
            // 尝试将当前元素尝试插入队尾
            que.push(nums[i]);
            // 队首元素为 当前区间最大值
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
