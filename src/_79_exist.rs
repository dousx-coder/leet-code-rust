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
        let mut board = board;
        let m = board.len();
        let n = board[0].len();

        let word = word.chars().collect::<Vec<char>>();
        let mut curr = vec![];
        // init use_flag
        let mut use_flag = vec![];
        for i in 0..m {
            let mut vec = vec![];
            for j in 0..n {
                vec.push(0);
            }
            use_flag.push(vec);
        }
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word[0] {
                    if Self::backtracking(i, j, &mut curr, m, n, &mut use_flag, &board, &word) {
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
        curr: &mut Vec<char>,
        m: usize,
        n: usize,
        used: &mut Vec<Vec<usize>>,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
    ) -> bool {
        if i >= m || j >= n || used[i][j] == 1 || curr.len() > word.len() {
            return false;
        }
        // 单词必须按照字母顺序，通过相邻的单元格内的字母构成，
        // 其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。
        // 同一个单元格内的字母不允许被重复使用。
        let x = board[i][j];
        let next_index = curr.len();
        if next_index >= word.len() {
            return false;
        }
        let w = word[next_index];
        if x == w {
            curr.push(x);
            used[i][j] = 1;
            if curr.len() == word.len() {
                if curr == word {
                    return true;
                }
            }
            // 尝试上下左右四个方向各走一次
            if Self::backtracking(i + 1, j, curr, m, n, used, board, word) {
                return true;
            }
            if Self::backtracking(i, j + 1, curr, m, n, used, board, word) {
                return true;
            }
            if i >= 1 && Self::backtracking(i - 1, j, curr, m, n, used, board, word) {
                return true;
            }
            if j >= 1 && Self::backtracking(i, j - 1, curr, m, n, used, board, word) {
                return true;
            }
            curr.pop();
            used[i][j] = 0;
        }
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
