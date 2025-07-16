///
/// [378.有序矩阵中第k小的元素](https://leetcode.cn/problems/kth-smallest-element-in-a-sorted-matrix/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::binary_search(matrix, k)
    }

    /// 二分查找
    fn binary_search(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut left = matrix[0][0];
        let mut right = matrix[n - 1][n - 1];
        while left < right {
            let mid = left + ((right - left) >> 1);
            let mut count = 0;
            for i in (0..n).rev() {
                for j in 0..n {
                    if matrix[i][j] <= mid {
                        count += 1;
                    }
                }
            }
            if count < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
    /// 暴力搜索
    fn brute_force(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        let mut matrix = matrix;
        let n = matrix.len();
        for m in 1..=k {
            ans = i32::MAX;
            // matrix行列是n x n
            // 每一行从左到右都是非递减顺序
            // 每一列从上到下都是非递减顺序
            for i in 0..n {
                for j in 0..n {
                    if matrix[i][j] == i32::MAX {
                        // 已经遍历过
                        continue;
                    }
                    if matrix[i][j] > ans {
                        break;
                    }
                    if matrix[i][j] <= ans {
                        // 记录当前循环的最小值以及坐标
                        ans = matrix[i][j];
                        x = i;
                        y = j;
                    }
                }
            }
            // 标记已遍历
            matrix[x][y] = i32::MAX;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
        assert_eq!(
            Solution::brute_force(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::brute_force(vec![vec![-5]], 1), -5);
        assert_eq!(Solution::binary_search(vec![vec![-5]], 1), -5);
    }
}
