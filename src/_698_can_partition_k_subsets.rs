/// 698 划分为K个相等的子集
///
/// https://leetcode.cn/problems/partition-to-k-equal-sum-subsets/
struct Solution;
impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let x = nums.iter().sum::<i32>();
        if x % k != 0 {
            return false;
        }
        // 平均数
        let avg = x / k;
        let mut nums = nums;
        // 倒序排序
        nums.sort_by(|a, b| b.cmp(a));
        let buckets = &mut vec![avg; k as usize];
        Self::backtracking(0, buckets, &nums)
    }
    fn backtracking(index: usize, buckets: &mut Vec<i32>, nums: &Vec<i32>) -> bool {
        if index >= nums.len() {
            //数字都分完了
            return true;
        }
        // 寻找nums[index] 应该放到哪个桶中
        for j in 0..buckets.len() {
            if nums[index] > buckets[j] {
                continue;
            }
            // 桶中减去当前数字
            // 减去nums[index]之后，buckets[j]最小也只能是0(上面有判断)，
            // 如果刚好是0(题意中的nums取值均大于0，所以下次循环该桶会被跳过)
            buckets[j] -= nums[index];
            if Self::backtracking(index + 1, buckets, nums) {
                return true;
            }
            // 回溯
            buckets[j] += nums[index];
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4),
            true
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3),
            false
        )
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::can_partition_k_subsets(vec![1, 1, 1, 1, 2, 2, 2, 2], 2),
            true
        )
    }
    #[test]
    fn t4() {
        assert_eq!(
            Solution::can_partition_k_subsets(
                vec![3, 3, 10, 2, 6, 5, 10, 6, 8, 3, 2, 1, 6, 10, 7, 2],
                6
            ),
            false
        )
    }
    #[test]
    fn t5() {
        assert_eq!(
            Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5], 4),
            false
        )
    }
    #[test]
    fn t6() {
        assert_eq!(
            Solution::can_partition_k_subsets(
                vec![9, 10, 14, 8, 15, 7, 15, 12, 15, 13, 10, 14, 9, 11, 9, 14],
                5
            ),
            true
        )
    }
}
