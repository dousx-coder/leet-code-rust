/// 45 跳跃游戏 II
///
///  https://leetcode.cn/problems/jump-game-ii/description/
struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // 贪心
        // 从前往后遍历：每次在当前能跳到的范围内，选择能跳得最远的位置作为下一步的起点。
        let mut step = 0;
        // 当前能跳到的最远位置。
        let mut max_pos = 0;
        // 当前跳跃的边界，当遍历到边界时，更新步数并设置新的边界。
        let mut end = 0;
        for i in 0..nums.len() - 1 {
            max_pos = max_pos.max(i + nums[i] as usize);
            if i == end {
                step += 1;
                end = max_pos;
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
