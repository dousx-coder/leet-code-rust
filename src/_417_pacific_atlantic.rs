///
/// [417. 太平洋与大西洋水流问题](https://leetcode.cn/problems/pacific-atlantic-water-flow/?envType=problem-list-v2&envId=breadth-first-search)
///
struct Solution;

impl Solution {
    /// 有一个 `m × n` 的矩形岛屿，与 太平洋 和 大西洋 相邻。 "太平洋" 处于大陆的左边界和上边界，而 "大西洋" 处于大陆的右边界和下边界。
    ///
    /// 这个岛被分割成一个由若干方形单元格组成的网格。给定一个 `m x n` 的整数矩阵 `heights` ， `heights[r][c]` 表示坐标 `(r, c)` 上单元格 高于海平面的高度 。
    ///
    /// 岛上雨水较多，如果相邻单元格的高度 小于或等于 当前单元格的高度，雨水可以直接向北、南、东、西流向相邻单元格。水可以从海洋附近的任何单元格流入海洋。
    ///
    /// 雨水只能从高的地方流向低的地方，求二维表格中哪些坐标的雨水既可以流入太平洋又可以流入大西洋。
    ///
    /// 返回网格坐标 `result` 的 2D 列表 ，其中 `result[i] = [ri, ci]` 表示雨水从单元格 `(ri, ci)` 流动 既可流向太平洋也可流向大西洋 。
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty() || heights[0].is_empty() {
            return vec![];
        }

        let (m, n) = (heights.len(), heights[0].len());
        // 创建两个二维数组，分别表示是否可以流向太平洋和大西洋
        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // 从太平洋边界开始DFS
        for i in 0..m {
            Self::dfs(&heights, &mut pacific, &directions, i, 0, m, n);
        }
        for j in 0..n {
            Self::dfs(&heights, &mut pacific, &directions, 0, j, m, n);
        }

        // 从大西洋边界开始DFS
        for i in 0..m {
            Self::dfs(&heights, &mut atlantic, &directions, i, n - 1, m, n);
        }
        for j in 0..n {
            Self::dfs(&heights, &mut atlantic, &directions, m - 1, j, m, n);
        }

        let mut result = vec![];
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }

    fn dfs(
        heights: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &[(i32, i32); 4],
        row: usize,
        col: usize,
        m: usize,
        n: usize,
    ) {
        if visited[row][col] {
            return;
        }

        visited[row][col] = true;

        for &(dx, dy) in directions {
            let next_row = row as i32 + dx;
            let next_col = col as i32 + dy;

            if next_row >= 0
                && next_row < m as i32
                && next_col >= 0
                && next_col < n as i32
                && !visited[next_row as usize][next_col as usize]
                && heights[next_row as usize][next_col as usize] >= heights[row][col]
            {
                Self::dfs(
                    heights,
                    visited,
                    directions,
                    next_row as usize,
                    next_col as usize,
                    m,
                    n,
                );
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
            Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ]),
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::pacific_atlantic(vec![vec![2, 1], vec![1, 2],]),
            vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::pacific_atlantic(vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]),
            vec![
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
                vec![2, 2]
            ]
        );
    }
}
