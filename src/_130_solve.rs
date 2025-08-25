///
/// [130. 被围绕的区域](https://leetcode.cn/problems/surrounded-regions/)
///
struct Solution;
impl Solution {
    /// 从边缘的 O开始遍历，遇到相连的的O改成F,
    ///
    /// 最后剩下的O都是被围绕的区域，再将O改成X，将A改成O
    pub fn solve(board: &mut Vec<Vec<char>>) {
        //  m行n列
        let m = board.len();
        // 二维数组长度一样，这里取第0个即可
        let n = board[0].len();
        if m < 3 || n < 3 {
            return;
        }

        for row in 0..m {
            // 遍历第一列和最后一列
            Self::fire(row, 0, m, n, board);
            Self::fire(row, n - 1, m, n, board);
        }

        for col in 0..n {
            // 遍历第一行和最后一行
            Self::fire(0, col, m, n, board);
            Self::fire(m - 1, col, m, n, board);
        }
        for row in 0..m {
            for col in 0..n {
                if board[row][col] == 'F' {
                    board[row][col] = 'O';
                    continue;
                }
                if board[row][col] == 'O' {
                    board[row][col] = 'X';
                }
            }
        }
    }

    fn fire(row: usize, col: usize, m: usize, n: usize, board: &mut Vec<Vec<char>>) {
        if row >= m || col >= n || board[row][col] != 'O' {
            return;
        }
        board[row][col] = 'F';
        Self::fire(row + 1, col, m, n, board);
        Self::fire(row, col + 1, m, n, board);
        if let Some(sub) = col.checked_sub(1) {
            Self::fire(row, sub, m, n, board);
        }
        if let Some(sub) = row.checked_sub(1) {
            Self::fire(sub, col, m, n, board);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        )
    }
    #[test]
    fn t2() {
        let mut board = vec![
            vec!['X', 'O', 'X', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'O', 'X', 'O', 'X', 'O'],
                vec!['O', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'O'],
                vec!['O', 'X', 'O', 'X', 'O', 'X']
            ]
        )
    }
}
