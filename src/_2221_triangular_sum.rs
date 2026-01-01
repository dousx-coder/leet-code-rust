///
/// [2221. 数组的三角和](https://leetcode.cn/problems/find-triangular-sum-of-an-array/?envType=problem-list-v2&envId=combinatorics)
///
struct Solution;
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        while nums.len() > 1 {
            let mut next = vec![];
            for i in 0..nums.len() - 1 {
                next.push((nums[i] + nums[i + 1]) % 10);
            }
            nums = next;
        }

        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::triangular_sum(vec![1, 2, 3, 4, 5]);
        assert_eq!(ans, 8);
    }

    #[test]
    fn t2() {
        let ans = Solution::triangular_sum(vec![5]);
        assert_eq!(ans, 5);
    }
}
