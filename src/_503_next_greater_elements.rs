/// 503 下一个更大元素 II
///     
/// https://leetcode.cn/problems/next-greater-element-ii/
struct Solution;
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![0];
        let len = nums.len();
        let mut ans = vec![-1; len];

        for i in 1..2 * len {
            let i = i % len;
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
                let j = stack.pop().unwrap();
                ans[j] = nums[i];
            }
            stack.push(i);
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
