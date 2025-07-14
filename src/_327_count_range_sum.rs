///
/// [327. 区间和的个数](https://leetcode.cn/problems/count-of-range-sum/description/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        let n = nums.len();
        // 给前缀和数组做归并排序，不是借助归并排序后元素的有序性，
        // 而是在归并过程中，所有右边元素的原始下标必然大于所有左边元素的原始下标
        let mut prefix_sum = vec![nums[0]];
        for i in 1..n {
            prefix_sum.push(prefix_sum[i - 1] + nums[i]);
        }
        Self::count(&mut prefix_sum, 0, n - 1, lower as i64, upper as i64)
    }
    fn count(prefix_sum: &mut Vec<i64>, left: usize, right: usize, lower: i64, upper: i64) -> i32 {
        if left == right {
            // 区间里只有一个值，判断当前值是否在[lower,upper]范围内
            return if prefix_sum[left] >= lower && prefix_sum[left] <= upper {
                1
            } else {
                0
            };
        }
        let mid = left + ((right - left) >> 1);
        let count_left = Self::count(prefix_sum, left, mid, lower, upper);
        let count_right = Self::count(prefix_sum, mid + 1, right, lower, upper);
        let count_merge = Self::merge(prefix_sum, left, mid, right, lower, upper);
        count_left + count_right + count_merge
    }
    fn merge(
        prefix_sum: &mut Vec<i64>,
        left: usize,
        mid: usize,
        right: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        // left<=right的情况下,满足lower ≤ prefix_sum[i] - prefix_sum[j] ≤ upper一组解
        // ①: prefix_sum[i] - prefix_sum[j] ≥ lower
        // ②: prefix_sum[i] - prefix_sum[j] ≤ upper
        // 由公式①可得: prefix_sum[j] ≤ prefix_sum[i] - lower
        // 由公式②可得: prefix_sum[j] ≥ prefix_sum[i] - upper
        // 合并得到: prefix_sum[i] - upper ≤ prefix_sum[j] ≤ prefix_sum[i] - lower
        // 得到查找区间[prefix_sum[i] - upper, prefix_sum[i] - lower]
        let mut ans = 0;
        let mut windows_left = left;
        let mut windows_right = left;
        for i in mid + 1..=right {
            while windows_left <= mid && prefix_sum[windows_left] < prefix_sum[i] - upper {
                // windows_left 找到第一个 ≥ new_upper 的位置
                windows_left += 1;
            }
            while windows_right <= mid && prefix_sum[windows_right] <= prefix_sum[i] - lower {
                // windows_right 找到第一个 > new_lower 的位置
                windows_right += 1;
            }
            ans += windows_right - windows_left;
        }
        Self::sort(prefix_sum, left, mid, right);
        ans as i32
    }

    fn sort(prefix_sum: &mut Vec<i64>, left: usize, mid: usize, right: usize) {
        let mut helper = vec![];
        let mut p1 = left;
        let mut p2 = mid + 1;
        while p1 <= mid && p2 <= right {
            helper.push(if prefix_sum[p1] <= prefix_sum[p2] {
                p1 += 1;
                prefix_sum[p1 - 1]
            } else {
                p2 += 1;
                prefix_sum[p2 - 1]
            });
        }
        while p1 <= mid {
            helper.push(prefix_sum[p1]);
            p1 += 1;
        }
        while p2 <= right {
            helper.push(prefix_sum[p2]);
            p2 += 1;
        }
        for i in 0..(right - left + 1) {
            prefix_sum[left + i] = helper[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::count_range_sum(vec![0, 0], 0, 0), 3);
    }
    #[test]
    fn t4() {
        assert_eq!(
            Solution::count_range_sum(vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864),
            3
        );
    }
}
