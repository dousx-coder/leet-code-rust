///
/// [327. 区间和的个数](https://leetcode.cn/problems/count-of-range-sum/description/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        let n = nums.len();
        let mut sums = vec![nums[0]];
        for i in 1..n {
            sums.push(sums[i - 1] + nums[i]);
        }
        Self::count(&mut sums, 0, n - 1, lower as i64, upper as i64)
    }
    fn count(sums: &mut Vec<i64>, left: usize, right: usize, lower: i64, upper: i64) -> i32 {
        if left == right {
            return if sums[left] >= lower && sums[left] <= upper {
                1
            } else {
                0
            };
        }
        let mid = left + ((right - left) >> 1);
        let count_left = Self::count(sums, left, mid, lower, upper);
        let count_right = Self::count(sums, mid + 1, right, lower, upper);
        let count_merge = Self::merge(sums, left, mid, right, lower, upper);
        count_left + count_right + count_merge
    }
    fn merge(
        sums: &mut Vec<i64>,
        left: usize,
        mid: usize,
        right: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        let mut ans = 0;
        let mut windows_left = left;
        let mut windows_right = left;
        for i in mid + 1..=right {
            let new_low = sums[i] - upper;
            let new_up = sums[i] - lower;
            while windows_left <= mid && sums[windows_left] < new_low {
                windows_left += 1;
            }
            while windows_right <= mid && sums[windows_right] <= new_up {
                windows_right += 1;
            }
            ans += windows_right - windows_left;
        }
        let mut helper = vec![];

        let mut p1 = left;
        let mut p2 = mid + 1;
        while p1 <= mid && p2 <= right {
            helper.push(if sums[p1] <= sums[p2] {
                p1 += 1;
                sums[p1 - 1]
            } else {
                p2 += 1;
                sums[p2 - 1]
            });
        }
        while p1 <= mid {
            helper.push(sums[p1]);
            p1 += 1;
        }
        while p2 <= right {
            helper.push(sums[p2]);
            p2 += 1;
        }
        for i in 0..(right - left + 1) {
            sums[left + i] = helper[i];
        }
        ans as i32
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
