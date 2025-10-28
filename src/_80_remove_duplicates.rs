///
/// [80. 删除有序数组中的重复项Ⅱ](https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/description/)
///
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }

        let mut slow = 2;
        let mut fast = 2;

        while fast < n {
            // 只有当当前元素与前两个元素不同时才保留
            if nums[fast] != nums[slow - 2] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        nums.truncate(slow);
        slow as i32
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
