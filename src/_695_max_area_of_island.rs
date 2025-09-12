///
/// [695. 岛屿最大的面积](https://leetcode.cn/problems/max-area-of-island/)
///
struct Solution;
impl Solution {
    /// 给你一个大小为 `m x n` 的二进制矩阵 `grid` 。
    ///
    ///岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在 水平或者竖直的四个方向上 相邻。你可以假设 `grid` 的四个边缘都被 0（代表水）包围着。
    ///
    ///岛屿的面积是岛上值为 1 的单元格的数目。
    ///
    ///计算并返回 `grid` 中最大的岛屿面积。如果没有岛屿，则返回面积为 0 。
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    let area = Self::dfs(&mut grid, i, j);
                    max = max.max(area);
                }
            }
        }
        max
    }
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if i >= grid.len() || j >= grid[i].len() || grid[i][j] == 0 {
            return 0;
        }
        grid[i][j] = 0;
        1 + Self::dfs(grid, i + 1, j)
            + Self::dfs(grid, i, j + 1)
            + match i.checked_sub(1) {
                Some(row) => Self::dfs(grid, row, j),
                None => 0,
            }
            + match j.checked_sub(1) {
                Some(col) => Self::dfs(grid, i, col),
                None => 0,
            }
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_area_of_island() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);
    }
}
