///
/// [200. 岛屿数量](https://leetcode.cn/problems/number-of-islands/description/?envType=problem-list-v2&envId=depth-first-search)
///
struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        Self::dfs(grid)
        // Self::bfs(grid)
    }

    fn dfs(grid: Vec<Vec<char>>) -> i32 {
        //  m行n列
        let m = grid.len();
        // 二维数组长度一样，这里取第0个即可
        let n = grid[0].len();
        let mut fire: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if fire[i][j] {
                    continue;
                }
                if Self::is_land(&grid[i][j]) {
                    //开始放火
                    Self::fire(i, j, m, n, &mut fire, &grid);
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
        if Self::is_land(&grid[row][col]) {
            fire[row][col] = true;
            Self::fire(row + 1, col, m, n, fire, grid);
            Self::fire(row, col + 1, m, n, fire, grid);
            if let Some(sub) = row.checked_sub(1) {
                Self::fire(sub, col, m, n, fire, grid);
            }
            if let Some(sub) = col.checked_sub(1) {
                Self::fire(row, sub, m, n, fire, grid);
            };
        } else {
            return;
        }
    }
    fn is_land(ch: &char) -> bool {
        *ch == '1'
    }

    fn bfs(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut count = 0;

        // 定义四个方向：上下左右
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for i in 0..m {
            for j in 0..n {
                // 如果当前是陆地且未访问过
                if grid[i][j] == '1' && !visited[i][j] {
                    count += 1;
                    let mut queue = std::collections::VecDeque::new();
                    queue.push_back((i, j));
                    visited[i][j] = true;
                    // bfs,将(i,j)四周的陆地节点都标记为已访问
                    while let Some((x, y)) = queue.pop_front() {
                        // 向四个方向扩展
                        for &(dx, dy) in &directions {
                            let next_x = x as isize + dx;
                            let next_y = y as isize + dy;

                            // 检查边界条件
                            if next_x >= 0
                                && next_x < m as isize
                                && next_y >= 0
                                && next_y < n as isize
                                && grid[next_x as usize][next_y as usize] == '1'
                                && !visited[next_x as usize][next_y as usize]
                            {
                                queue.push_back((next_x as usize, next_y as usize));
                                visited[next_x as usize][next_y as usize] = true;
                            }
                        }
                    }
                }
            }
        }
        count
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

    #[test]
    fn t3() {
        assert_eq!(
            Solution::bfs(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
