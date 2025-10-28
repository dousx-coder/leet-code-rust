///
/// [80. 删除有序数组中的重复项Ⅱ](https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/description/)
///
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        let mut pre_count = 1;
        let mut pre_num = nums[0];

        while i < nums.len() {
            if nums[i] == pre_num {
                pre_count += 1;
            } else {
                pre_count = 1;
                pre_num = nums[i];
            }

            if pre_count > 2 {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn t2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
