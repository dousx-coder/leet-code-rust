/// 240 搜索二维矩阵Ⅱ
///
/// https://leetcode.cn/problems/search-a-2d-matrix-ii/?envType=problem-list-v2&envId=binary-search
struct Solution;
impl Solution {
    ///
    /// 每行的元素从左到右升序排列。
    ///
    /// 每列的元素从上到下升序排列。
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        //  m行，n列
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row = 0;
        let mut col = n - 1;
        loop {
            if matrix[row][col] == target {
                return true;
            }
            if matrix[row][col] < target {
                row += 1;
                if row >= m {
                    return false;
                }
            } else {
                if col < 1 {
                    return false;
                }
                col -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 5;
        assert!(Solution::search_matrix(matrix, target));
    }
    #[test]
    fn t2() {
        assert!(!Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            20
        ));
    }
    #[test]
    fn t3() {
        assert!(Solution::search_matrix(vec![vec![-1, 3]], 3));
    }
}
