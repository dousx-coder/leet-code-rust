///
/// [300. 最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    /// ### 1. 维护一个数组 sub，其中 `sub[i]` 表示长度为 i+1 的 LIS 的最小可能尾部值。
    /// ### 2. 遍历原始数组 nums 中的每个元素 num：
    ///  - 如果 num 大于 sub 的最后一个元素，则将其添加到 sub 中（形成更长的 LIS）。
    ///  - 否则，使用 二分查找 找到 sub 中第一个大于等于 num 的位置，并将该位置的值替换为 num，以保证更优的 LIS 尾部值。
    /// ### 3. 最终 sub 的长度即为最长递增子序列的长度
    ///
    /// 时间复杂度为 O(n log n)
    ///
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub = Vec::new();
        for &num in &nums {
            // 二分查找插入位置
            let idx = sub.binary_search(&num).unwrap_or_else(|x| x);
            if idx == sub.len() {
                // 可以扩展 LIS 长度
                sub.push(num);
            } else {
                // 替换更小的尾部值
                sub[idx] = num;
            }
        }
        sub.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        // 子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。
        // 例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
    #[test]
    fn t4() {
        assert_eq!(Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    }
}
