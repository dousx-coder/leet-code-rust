///
///[456. 132模式](https://leetcode.cn/problems/132-pattern/?envType=problem-list-v2&envId=monotonic-stack)
///
struct Solution;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let len = nums.len();
        if len < 3 {
            return false;
        }
        // 记录132模式中间的数字(3对应的数字)
        let mut mode3 = vec![];
        // 记录132模式最后一位(2对应的数字)
        let mut mode2 = i32::MIN;
        // 倒叙
        for i in (0..len).rev() {
            if nums[i] < mode2 {
                return true;
            }
            while !mode3.is_empty() && *mode3.last().unwrap() < nums[i] {
                mode2 = mode3.pop().unwrap();
            }
            mode3.push(nums[i]);
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
    }
}
