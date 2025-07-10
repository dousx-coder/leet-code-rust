///
/// [15. 三数之和](https://leetcode.cn/problems/3sum/)
struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ans = Vec::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                return ans;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                // 去重
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                    continue;
                }
                if sum < 0 {
                    left += 1;
                    continue;
                }
                //  sum == 0
                ans.push(vec![nums[i], nums[left], nums[right]]);
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
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
        let ans = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(ans, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
