///
/// 31 下一个排列
///
/// https://leetcode.cn/problems/next-permutation/description/
struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }
        let n = nums.len();
        for i in (0..n).rev() {
            for j in (i + 1..n).rev() {
                if nums[i] < nums[j] {
                    nums.swap(i, j);
                    nums[i + 1..].reverse();
                    return;
                }
            }
        }
        // 走到这里说明数组是降序
        nums.reverse();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // 1 2 3
        // 1 3 2
        // 2 1 3
        // 2 3 1
        // 3 1 2
        // 3 2 1
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }
    #[test]
    fn t2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
