///
/// [315. 计算右侧小于当前元素的个数](https://leetcode.cn/problems/count-of-smaller-numbers-after-self/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        Self::merge_count_smaller(nums)
    }
    /// 二分检索（超时）
    fn binary_search(nums: Vec<i32>) -> Vec<i32> {
        let mut auxiliary = vec![];
        let mut result = vec![];
        for i in (0..nums.len()).rev() {
            let x = Self::binary(&auxiliary, nums[i]);
            result.insert(0, x as i32);
            auxiliary.insert(x, nums[i]);
        }
        result
    }

    fn binary(auxiliary: &Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = auxiliary.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if auxiliary[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
    ///归并排序法
    fn merge_count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut indices = (0..nums.len()).collect::<Vec<_>>();
        let mut count = vec![0; nums.len()];
        Self::merge_sort(&nums, &mut indices, 0, nums.len(), &mut count);
        count
    }

    fn merge_sort(nums: &[i32], indices: &mut [usize], l: usize, r: usize, count: &mut [i32]) {
        if r - l <= 1 {
            return;
        }
        let m = (l + r) / 2;
        Self::merge_sort(nums, indices, l, m, count);
        Self::merge_sort(nums, indices, m, r, count);

        let mut merged = Vec::with_capacity(r - l);
        let (mut i, mut j) = (l, m);

        // 归并过程统计右侧小于当前元素的数量
        while i < m && j < r {
            if nums[indices[i]] > nums[indices[j]] {
                merged.push(indices[j]);
                j += 1;
            } else {
                let idx = indices[i];
                *count.get_mut(idx).unwrap() += (j - m) as i32;
                merged.push(indices[i]);
                i += 1;
            }
        }

        while i < m {
            let idx = indices[i];
            *count.get_mut(idx).unwrap() += (j - m) as i32;
            merged.push(indices[i]);
            i += 1;
        }

        while j < r {
            merged.push(indices[j]);
            j += 1;
        }

        for k in 0..merged.len() {
            indices[l + k] = merged[k];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::binary_search(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(
            Solution::merge_count_smaller(vec![5, 2, 6, 1]),
            vec![2, 1, 1, 0]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }
}
