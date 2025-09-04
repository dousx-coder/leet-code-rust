///
/// [463. 岛屿的周长](https://leetcode.cn/problems/island-perimeter/?envType=problem-list-v2&envId=depth-first-search)
///
struct Solution;
impl Solution {
    /// `grid[i][j] = 1` 表示陆地， `grid[i][j]` = 0 表示水域
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        Self::calculate(grid)
    }

    /// 迭代解法
    fn calculate(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut perimeter = 0;

        // 直接遍历网格
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 1 {
                    // 每个陆地格子初始贡献4条边
                    perimeter += 4;

                    // 如果上方有陆地，减少2条边（上下相邻各自减少1条边）
                    if i > 0 && grid[i - 1][j] == 1 {
                        perimeter -= 2;
                    }

                    // 如果左方有陆地，减少2条边（左右相邻各自减少1条边）
                    if j > 0 && grid[i][j - 1] == 1 {
                        perimeter -= 2;
                    }
                }
            }
        }

        perimeter
    }
    /// 递归解法
    pub fn recursive(grid: Vec<Vec<i32>>) -> i32 {
        // row x col
        let row = grid.len();
        let col = grid[0].len();
        let mut perimeter = 0;
        let mut flag = vec![vec![false; col]; row];

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 1 {
                    // 岛是四个边，如果某个边遇到边界则周长+1
                    // 最多存在1个岛，所以递归结束后直接返回
                    // 开始上下左右四个方向搜索，如果相邻为陆地或者海洋，则周长加1，反之继续递归
                    Self::fire(&grid, &mut flag, &mut perimeter, row, col, i, j);
                    return perimeter;
                }
            }
        }
        perimeter
    }
    fn fire(
        grid: &Vec<Vec<i32>>,
        flag: &mut Vec<Vec<bool>>,
        perimeter: &mut i32,
        row: usize,
        col: usize,
        x: usize,
        y: usize,
    ) {
        if flag[x][y] {
            return;
        }

        if grid[x][y] == 1 {
            flag[x][y] = true;
            if x + 1 < row {
                Self::fire(grid, flag, perimeter, row, col, x + 1, y);
            } else {
                *perimeter += 1;
            }
            if y + 1 < col {
                Self::fire(grid, flag, perimeter, row, col, x, y + 1);
            } else {
                *perimeter += 1;
            }

            if let Some(sub) = x.checked_sub(1) {
                Self::fire(grid, flag, perimeter, row, col, sub, y);
            } else {
                *perimeter += 1;
            }
            if let Some(sub) = y.checked_sub(1) {
                Self::fire(grid, flag, perimeter, row, col, x, sub);
            } else {
                *perimeter += 1;
            }
        } else {
            *perimeter += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::calculate(vec![vec![1]]), 4);
        assert_eq!(Solution::recursive(vec![vec![1]]), 4);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
    }
}
