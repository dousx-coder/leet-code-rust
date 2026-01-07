///
/// [324. 摆动排序Ⅱ](https://leetcode.cn/problems/wiggle-sort-ii/?envType=problem-list-v2&envId=sorting)
///
struct Solution;
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut sorted = nums.clone();
        sorted.sort_unstable();

        let n = nums.len();
        let mid = (n - 1) / 2;

        let mut i = 0;
        // 先填充奇数位置（从中间开始向前取）
        for j in (0..=mid).rev() {
            nums[i] = sorted[j];
            i += 2;
        }

        // 再填充偶数位置（从末尾开始向前取）
        i = 1;
        for j in (mid + 1..n).rev() {
            nums[i] = sorted[j];
            i += 2;
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        //  1 1 1 4 5 6
        let mut nums = vec![1, 5, 1, 1, 6, 4];
        Solution::wiggle_sort(&mut nums);
        println!("{:?}", nums);
        assert!(vec![1, 6, 1, 5, 1, 4] == nums || vec![1, 4, 1, 5, 1, 6] == nums);
    }

    #[test]
    fn t2() {
        let mut nums = vec![1, 3, 2, 2, 3, 1];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(vec![2, 3, 1, 3, 1, 2], nums);
    }
}
