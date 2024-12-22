///
/// https://leetcode.cn/problems/remove-duplicates-from-sorted-array/description/
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut fast = 1;
        let mut slow = 1;
        while fast < nums.len() {
            if nums[fast] != nums[fast - 1] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    }
}
