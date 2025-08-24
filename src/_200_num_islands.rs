///
/// [200. 岛屿数量](https://leetcode.cn/problems/number-of-islands/description/?envType=problem-list-v2&envId=depth-first-search)
///
struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut fire: Vec<Vec<bool>> = vec![];
        //  m行n列
        let m = grid.len();
        // 二维数组长度一样，这里取第0个即可
        let n = grid[0].len();
        for i in 0..grid.len() {
            fire.push(vec![false; grid[i].len()]);
        }
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if fire[i][j] {
                    continue;
                }
                if Solution::is_land(&grid[i][j]) {
                    //开始放火
                    Solution::fire(i, j, m, n, &mut fire, &grid);
                    count += 1;
                }
            }
        }
        count
    }
    fn fire(
        row: usize,
        col: usize,
        m: usize,
        n: usize,
        fire: &mut Vec<Vec<bool>>,
        grid: &Vec<Vec<char>>,
    ) {
        if row >= m || col >= n {
            return;
        }
        if fire[row][col] {
            return;
        }
        if Solution::is_land(&grid[row][col]) {
            fire[row][col] = true;
            Solution::fire(row + 1, col, m, n, fire, grid);
            Solution::fire(row, col + 1, m, n, fire, grid);
            if let Some(sub) = row.checked_sub(1) {
                Solution::fire(sub, col, m, n, fire, grid);
            }
            if let Some(sub) = col.checked_sub(1) {
                Solution::fire(row, sub, m, n, fire, grid);
            };
        } else {
            return;
        }
    }
    fn is_land(ch: &char) -> bool {
        *ch == '1'
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
