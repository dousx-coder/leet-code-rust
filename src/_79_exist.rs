///
/// 79 单词搜索
///
/// https://leetcode.cn/problems/word-search/description/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() {
            return false;
        }
        let m = board.len();
        let n = board[0].len();
        let word = word.chars().collect::<Vec<char>>();
        let mut used = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word[0] {
                    if Self::backtracking(i, j, 0, m, n, &mut used, &board, &word) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn backtracking(
        i: usize,
        j: usize,
        index: usize,
        m: usize,
        n: usize,
        used: &mut Vec<Vec<bool>>,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
    ) -> bool {
        if i >= m || j >= n || used[i][j] || index >= word.len() || board[i][j] != word[index] {
            return false;
        }
        if index == word.len() - 1 {
            return true;
        }
        used[i][j] = true;
        // 尝试上下左右四个方向各走一次
        if Self::backtracking(i + 1, j, index + 1, m, n, used, board, word)
            || Self::backtracking(i, j + 1, index + 1, m, n, used, board, word)
            || (i > 0 && Self::backtracking(i - 1, j, index + 1, m, n, used, board, word))
            || (j > 0 && Self::backtracking(i, j - 1, index + 1, m, n, used, board, word))
        {
            return true;
        }
        used[i][j] = false;
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn t2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }
    #[test]
    fn t3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        assert_eq!(Solution::exist(board, word), false);
    }
}
