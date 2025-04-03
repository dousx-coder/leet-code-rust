/// 45 跳跃游戏 II
///
///  https://leetcode.cn/problems/jump-game-ii/description/
struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // 贪心
        // 从最后一个元素开始，每次都找到能跳到当前元素之中下标最小的那个元素即可
        let mut i = nums.len() - 1;
        let mut step = 0;
        while i > 0 {
            for j in 0..i {
                // j从前往后判断，遇到能跳到当前元素的即是下标最小的
                if nums[j] >= (i - j) as i32 {
                    step += 1;
                    i = j;
                    break;
                }
            }
        }
        step
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
