///
/// [731. 乘积小于K的子数组](https://leetcode.cn/problems/subarray-product-less-than-k/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    ///
    /// 双指针法，如果一个子串的乘积小于k，那么他的每个子集都小于k，而一个长度为n的数组，他的所有连续子串数量是1+2+...n，但是会和前面的重复。
    /// 比如例子中[10, 5, 2, 6]，
    /// 第一个满足条件的子串是[ 10 ]，第二个满足的是[10, 5]，
    /// 但是第二个数组的子集[ 10 ]和前面的已经重复了，因此我们只需要计算包含最右边的数字的子串数量，就不会重复了，
    /// 也就是在计算[10, 5]这个数组的子串是，只加入[ 5 ]和[10, 5]，而不加入[ 10 ]，这部分的子串数量刚好是r - l + 1
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 || k == 0 {
            return 0;
        }
        let mut left = 0;
        let mut prod = 1;
        let mut ans = 0;
        for right in 0..nums.len() {
            prod *= nums[right];
            while prod >= k {
                prod /= nums[left];
                left += 1;
            }
            ans += right - left + 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }
}
