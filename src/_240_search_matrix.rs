/// 240 搜索二维矩阵Ⅱ
///
/// https://leetcode.cn/problems/search-a-2d-matrix-ii/?envType=problem-list-v2&envId=binary-search
struct Solution;
impl Solution {
    fn search(
        matrix: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        target: i32,
        row: usize,
        col: usize,
        m: usize,
        n: usize,
    ) -> bool {
        if row >= m || col >= n || visited[row][col] {
            return false;
        }
        if matrix[row][col] == target {
            return true;
        }
        // 标记已经访问
        visited[row][col] = true;
        if matrix[row][col] > target {
            // 向左上方查找
            // if 比checked_sub更高效
            (if row > 0 {
                Self::search(matrix, visited, target, row - 1, col, m, n)
            } else {
                false
            }) || (if col > 0 {
                Self::search(matrix, visited, target, row, col - 1, m, n)
            } else {
                false
            })
        } else {
            // 向右下方查找
            Self::search(matrix, visited, target, row + 1, col, m, n)
                || Self::search(matrix, visited, target, row, col + 1, m, n)
        }
    }

    ///
    /// 每行的元素从左到右升序排列。
    ///
    /// 每列的元素从上到下升序排列。
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut visited = vec![vec![false; n]; m];
        let row = m / 2;
        let col = n / 2;
        Self::search(&matrix, &mut visited, target, row, col, m, n)
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
    #[test]
    fn t4() {
        assert!(Solution::search_matrix(
            vec![
                vec![1, 3, 5, 7, 9],
                vec![2, 4, 6, 8, 10],
                vec![11, 13, 15, 17, 19],
                vec![12, 14, 16, 18, 20],
                vec![21, 22, 23, 24, 25]
            ],
            13
        ));
    }
}
