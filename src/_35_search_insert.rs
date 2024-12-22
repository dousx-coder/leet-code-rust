///
/// 35.搜索插入位置
///
/// https://leetcode.cn/problems/search-insert-position/description/

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::dichotomy(0, nums.len() - 1, &nums, target)
    }
    fn dichotomy(left: usize, right: usize, nums: &Vec<i32>, target: i32) -> i32 {
        if left == right {
            let lv = nums[left];
            return if lv >= target { left } else { left + 1 } as i32;
        }
        let middle_index = (left + right) / 2;
        let middle_value = nums[middle_index];
        if middle_value == target {
            middle_index as i32
        } else if middle_value > target {
            Self::dichotomy(left, middle_index, nums, target)
        } else {
            Self::dichotomy(middle_index + 1, right, nums, target)
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(2, result)
    }

    #[test]
    fn t2() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(1, result)
    }
    #[test]
    fn t3() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 7);
        assert_eq!(4, result)
    }
}
