/// [88. 合并两个有序数组](https://leetcode.cn/problems/merge-sorted-array/)
struct Solution;
impl Solution {
    /// 给你两个按 非递减顺序 排列的整数数组 `nums1` 和 `nums2`，另有两个整数 `m` 和 `n` ，分别表示 `nums1` 和 `nums2` 中的元素数目。
    ///
    /// 请你 合并 `nums2` 到 `nums1` 中，使合并后的数组同样按 非递减顺序 排列。
    ///
    /// 注意：最终，合并后数组不应由函数返回，而是存储在数组 `nums1` 中。
    /// 为了应对这种情况，`nums1` 的初始长度为 `m + n`，其中前 `m` 个元素表示应合并的元素，后 `n` 个元素为 `0` ，应忽略。
    /// nums2 的长度为 `n` 。
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let len = nums1.len();
        if len != m + n {
            panic!()
        }
        for i in m..len {
            nums1[i] = nums2[i - m];
        }
        nums1.sort();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        let mut v1 = vec![1, 2, 3, 0, 0, 0];
        let mut v2 = vec![2, 5, 6];
        Solution::merge(&mut v1, 3, &mut v2, 3);
        assert_eq!(v1, vec![1, 2, 2, 3, 5, 6])
    }
}
