/// 162 寻找峰值
///
/// https://leetcode.cn/problems/find-peak-element/description/?envType=problem-list-v2&envId=binary-search
struct Solution;
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // 题目要求nums[i]!=nums[i+1]，所以不可能存在“平峰”的情况
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] > nums[mid + 1] {
                //此时mid为下坡路，那么有可能自己本身就是山峰，或者在下山的过程中，所以right=mid而不能等于mid-1
                right = mid;
            } else {
                //反之说明此时mid为上坡路，既然是上坡，那么mid肯定不是山峰，所以left=mid+1
                left = mid + 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    }
    #[test]
    fn t2() {
        let ans = Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]);
        assert!(ans == 1 || ans == 5);
    }
}
