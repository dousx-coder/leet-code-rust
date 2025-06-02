/// 739  每日温度
///
/// https://leetcode.cn/problems/daily-temperatures/description/
struct Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // 单调栈
        let mut monotonic_stack = vec![];
        let len = temperatures.len();
        let mut ans = vec![0; len];
        monotonic_stack.push(0);
        for i in 1..len {
            while !monotonic_stack.is_empty() {
                let last_index = *monotonic_stack.last().unwrap();
                if temperatures[i] > temperatures[last_index] {
                    ans[last_index] = (i - last_index) as i32;
                    monotonic_stack.pop();
                } else {
                    break;
                }
            }
            monotonic_stack.push(i);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
