///
/// [164. 最大间距](https://leetcode.cn/problems/maximum-gap/?envType=problem-list-v2&envId=sorting)
///
struct Solution;
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 2 {
            return 0;
        }
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        let sub = max - min;
        // 桶因子
        let bucket_factor = std::cmp::max(1, sub / (len - 1) as i32);
        let mut bucket_size = (sub / bucket_factor + 1) as usize;
        // 存储 (桶内最小值，桶内最大值) 对， (-1, -1) 表示该桶是空的
        let mut bucket = vec![vec![-1, -1]; bucket_size];
        for i in 0..len {
            let idx = ((nums[i] - min) / bucket_factor) as usize;
            // 若区间为空，则此元素是第一次进来，则最大值和最小值都是它
            if bucket[idx][0] == -1 {
                bucket[idx][1] = nums[i];
                bucket[idx][0] = nums[i];
            } else {
                bucket[idx][0] = bucket[idx][0].min(nums[i]);
                bucket[idx][1] = bucket[idx][1].max(nums[i]);
            }
        }

        let mut max_gap = i32::MIN;

        // 上一个下标
        let mut pre_idx = 0;
        for i in 0..bucket_size {
            // 判断当前桶区间是否为空，空则跳出
            if bucket[i][0] == -1 {
                continue;
            }
            // 计算上一个桶区间的最大值，与当前桶区间的的最小值的差
            // 然后与目标值比较，取最大值然后替换max_gap值
            max_gap = max_gap.max(bucket[i][0] - bucket[pre_idx][1]);
            pre_idx = i;
        }

        max_gap
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::maximum_gap(vec![1, 1, 1, 1]), 0);
    }
}
