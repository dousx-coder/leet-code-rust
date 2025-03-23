/// 494 目标和
///
/// https://leetcode.cn/problems/target-sum/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        Self::backtracking(0, &nums, target)
    }
    fn backtracking(pos: usize, nums: &Vec<i32>, target: i32) -> i32 {
        if pos == nums.len() {
            return if target == 0 { 1 } else { 0 };
        }
        let mut ans = 0;
        ans += Self::backtracking(pos + 1, nums, target - nums[pos]);
        ans += Self::backtracking(pos + 1, nums, target + nums[pos]);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3);
        assert_eq!(5, ans);
    }

    #[test]
    fn t2() {
        let ans = Solution::find_target_sum_ways(vec![1], 1);
        assert_eq!(1, ans);
    }

    #[test]
    fn t3() {
        let ans = Solution::find_target_sum_ways(vec![1], 0);
        assert_eq!(0, ans);
    }

    #[test]
    fn t4() {
        let ans = Solution::find_target_sum_ways(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            1,
        );
        assert_eq!(524288, ans);
    }
}
