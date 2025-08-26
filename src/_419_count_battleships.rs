///
/// [419. 棋盘上的战舰](https://leetcode.cn/problems/battleships-in-a-board/)
///
struct Solution;
impl Solution {
    /// 舰队 只能水平或者垂直放置在 board 上。
    ///
    /// 换句话说，舰队只能按 1 x k（1 行，k 列）或 k x 1（k 行，1 列）的形状放置，
    ///
    /// 其中 k 可以是任意大小。两个舰队之间至少有一个水平或垂直的空格分隔 （即没有相邻的舰队）。
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        //  m行n列
        let m = board.len();
        // 二维数组长度一样，这里取第0个即可
        let n = board[0].len();
        let mut board = board;
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'X' {
                    ans += 1;
                    Self::dfs(i, j, m, n, &mut board);
                }
            }
        }
        ans
    }
    fn dfs(row: usize, col: usize, m: usize, n: usize, board: &mut Vec<Vec<char>>) {
        if row >= m || col >= n || board[row][col] != 'X' {
            return;
        }
        board[row][col] = 'F';
        Self::dfs(row + 1, col, m, n, board);
        Self::dfs(row, col + 1, m, n, board);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
            ]),
            2
        )
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::count_battleships(vec![vec!['.']]), 0)
    }
}
