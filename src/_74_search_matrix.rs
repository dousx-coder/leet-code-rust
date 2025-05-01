/// 74 搜索二维矩阵
///
/// https://leetcode.cn/problems/search-a-2d-matrix/?envType=problem-list-v2&envId=binary-search
struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // m x n
        let m = matrix.len();
        let n = matrix[0].len();

        let mut l = 0;
        let mut r = m * n - 1;

        while l <= r {
            let mid = (l + r) / 2;
            let i = mid / n;
            let j = mid % n;
            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] < target {
                l = mid + 1;
            } else {
                if mid < 1 {
                    return false;
                }
                r = mid - 1;
            }
        }
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::search_matrix(vec![vec![1]], 0), false);
    }
}
