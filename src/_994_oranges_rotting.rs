///
/// [994. 腐烂的橘子](https://leetcode.cn/problems/rotting-oranges/)
///
struct Solution;
impl Solution {
    ///
    /// 在给定的 `m x n` 网格 `grid` 中，每个单元格可以有以下三个值之一：
    ///
    ///值 `0` 代表空单元格；
    ///
    ///值 `1` 代表新鲜橘子；
    ///
    ///值 `2` 代表腐烂的橘子。
    ///
    ///每分钟，腐烂的橘子 周围 `4` 个方向上相邻 的新鲜橘子都会腐烂。
    ///
    /// 返回 直到单元格中没有新鲜橘子为止所必须经过的最小分钟数。如果不可能，返回 `-1` 。
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        if !grid.iter().any(|row| row.iter().any(|&cell| cell == 2)) {
            if grid.iter().any(|row| row.iter().any(|&cell| cell == 1)) {
                // 存在新鲜橘子，无法全部腐烂
                return -1;
            }
            // 不存在橘子，认为全部腐烂，经过的时间为0分钟
            return 0;
        }

        let m = grid.len();
        let n = grid[0].len();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut grid = grid;
        let mut minutes = 0;
        loop {
            // decay表示本轮被腐烂的橘子
            let mut decay = vec![vec![false; n]; m];
            // 每次循环表示经过1分钟
            minutes += 1;
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 2 && !decay[i][j] {
                        for (dx, dy) in directions {
                            let x = i as isize + dx;
                            let y = j as isize + dy;
                            if x >= 0
                                && x < grid.len() as isize
                                && y >= 0
                                && y < grid[i].len() as isize
                            {
                                let x = x as usize;
                                let y = y as usize;
                                if grid[x][y] == 1 {
                                    // 刚被感染的不能再本轮循环中感染其他橘子
                                    grid[x][y] = 2;
                                    decay[x][y] = true;
                                }
                            }
                        }
                    }
                }
            }
            if !decay.iter().any(|row| row.iter().any(|&cell| cell)) {
                // 本轮循环没有腐烂任何橘子
                if grid.iter().any(|row| row.iter().any(|&cell| cell == 1)) {
                    // 还存在新鲜橘子，无法全部腐烂
                    return -1;
                }
                // 本轮循环多加了1分钟，所以这里减去1
                return minutes - 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0]]), 0);
    }
}
