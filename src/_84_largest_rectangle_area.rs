///
/// [84. 柱状图中最大的矩形](https://leetcode.cn/problems/largest-rectangle-in-histogram/)
///
struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        // 首尾加0
        // 如果原数组是[2,4,6,8],不加0的话,不会触发计算过程
        heights.insert(0, 0);
        heights.push(0);
        // 单调栈
        let mut stack = vec![];
        let mut max_area = 0;
        for i in 0..heights.len() {
            while !stack.is_empty() && heights[*stack.last().unwrap()] > heights[i] {
                let h = heights[stack.pop().unwrap()];
                let w = if stack.is_empty() {
                    i
                } else {
                    i - *stack.last().unwrap() - 1
                } as i32;
                max_area = max_area.max(h * w);
            }
            stack.push(i);
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
