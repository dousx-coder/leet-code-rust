///
/// N皇后
///
///  https://leetcode.cn/problems/n-queens/description/
///

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 1 {
            return vec![vec!["Q".to_string()]];
        }
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut chessboard: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
        Self::recursion(n, &mut result, 0, &mut chessboard);
        result
    }

    fn recursion(
        queen_num: i32,
        result: &mut Vec<Vec<String>>,
        row: i32,
        chessboard: &mut Vec<Vec<i32>>,
    ) {
        if row == queen_num {
            let mut answer: Vec<String> = vec![];
            for ele in chessboard.iter() {
                let mut row_str = String::new();
                for &ele in ele.iter() {
                    row_str.push(if ele == 1 { 'Q' } else { '.' });
                }
                answer.push(row_str);
            }
            result.push(answer);
            return;
        }

        for col in 0..queen_num {
            if Self::is_safe(chessboard, row, col, queen_num) {
                chessboard[row as usize][col as usize] = 1;
                Self::recursion(queen_num, result, row + 1, chessboard);
                chessboard[row as usize][col as usize] = 0;
            }
        }
    }

    fn is_safe(chessboard: &Vec<Vec<i32>>, row: i32, col: i32, board_size: i32) -> bool {
        for i in 0..row {
            if chessboard[i as usize][col as usize] == 1 {
                return false;
            }
        }
        let mut i = row;
        let mut j = col;
        while i >= 0 && j >= 0 {
            if chessboard[i as usize][j as usize] == 1 {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        let mut i = row;
        let mut j = col;
        while i >= 0 && j < board_size {
            if chessboard[i as usize][j as usize] == 1 {
                return false;
            }
            i -= 1;
            j += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let vr = Solution::solve_n_queens(8);
        for answer in &vr {
            for row in answer {
                println!("{}", row);
            }
            println!();
        }
        let len = vr.len();
        assert_eq!(len, 92);
    }
}
