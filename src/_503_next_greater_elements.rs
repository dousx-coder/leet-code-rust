/// 503 下一个更大元素 II
///     
/// https://leetcode.cn/problems/next-greater-element-ii/
struct Solution;
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![-1; len];
        let mut stack = Vec::with_capacity(len);

        for i in 0..(2 * len) {
            let idx = i % len;

            while let Some(&top) = stack.last() {
                if nums[top] < nums[idx] {
                    ans[stack.pop().unwrap()] = nums[idx];
                } else {
                    break;
                }
            }
            if i < len {
                stack.push(idx);
            }
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
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }
}
