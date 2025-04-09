/// 55 跳跃游戏
///
/// https://leetcode.cn/problems/jump-game/description/
struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // 贪心
        // 从前往后遍历：每次在当前能跳到的范围内，选择能跳得最远的位置作为下一步的起点。
        let len = nums.len();
        // jump可以覆盖的范围
        let mut jump = vec![false; len];
        jump[0] = true;
        for i in 0..len {
            if !jump[i] {
                return false;
            }
            let len = nums.len();
            for j in i + 1..=i + nums[i] as usize {
                if j >= len - 1 {
                    // 可以覆盖到最后一个位置
                    return true;
                }
                jump[j] = true;
            }
        }
        jump[len - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
