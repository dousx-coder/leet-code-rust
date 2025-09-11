///
/// [417. 太平洋与大西洋水流问题](https://leetcode.cn/problems/pacific-atlantic-water-flow/?envType=problem-list-v2&envId=breadth-first-search)
///
struct Solution;
impl Solution {
    /// 有一个 `m × n` 的矩形岛屿，与 太平洋 和 大西洋 相邻。 “太平洋” 处于大陆的左边界和上边界，而 “大西洋” 处于大陆的右边界和下边界。
    ///
    /// 这个岛被分割成一个由若干方形单元格组成的网格。给定一个 `m x n` 的整数矩阵 `heights` ， `heights[r][c]` 表示坐标 `(r, c)` 上单元格 高于海平面的高度 。
    ///
    /// 岛上雨水较多，如果相邻单元格的高度 小于或等于 当前单元格的高度，雨水可以直接向北、南、东、西流向相邻单元格。水可以从海洋附近的任何单元格流入海洋。
    ///
    /// 雨水只能从高的地方流向低的地方，求二维表格中哪些坐标的雨水既可以流入太平洋又可以流入大西洋。
    ///
    /// 返回网格坐标 `result` 的 2D 列表 ，其中 `result[i] = [ri, ci]` 表示雨水从单元格 `(ri, ci)` 流动 既可流向太平洋也可流向大西洋 。
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());

        // 元组：下标0表示能到达左上，1表示能到达右下
        let mut connect = vec![vec![(false, false); n]; m];
        // 初始化四条边
        for i in 0..m {
            // 左边界可以到达太平洋
            connect[i][0].0 = true;
            // 右边界可以到达大西洋
            connect[i][n - 1].1 = true;
        }
        for j in 0..n {
            // 上边界可以到达太平洋
            connect[0][j].0 = true;
            // 下边界可以到达大西洋
            connect[m - 1][j].1 = true;
        }
        for i in 0..m {
            for j in 0..n {
                Self::dfs(&heights, &mut connect, i, j, (m, n));
            }
        }
        //  遍历visited值均为true的下标，加入到result中
        let mut result = vec![];
        for i in 0..m {
            for j in 0..n {
                if connect[i][j] == (true, true) {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }
    fn dfs(
        heights: &Vec<Vec<i32>>,
        connect: &mut Vec<Vec<(bool, bool)>>,
        row: usize,
        col: usize,
        (m, n): (usize, usize),
    ) {
        if row + 1 < m && !connect[row][col].1 && heights[row][col] >= heights[row + 1][col] {
            //  如果下一个节点是false，则判断后续节点是否都比当前节点大
            let mut flag = true;
            for i in row + 1..m {
                if heights[row][col] < heights[i][col] {
                    flag = false;
                    break;
                }
            }
            if flag {
                connect[row][col].1 = true;
            } else {
                if !connect[row + 1][col].1 {
                    Self::dfs(heights, connect, row + 1, col, (m, n));
                }
                connect[row][col].1 = connect[row + 1][col].1;
            }
        }
        if col + 1 < n && !connect[row][col].1 && heights[row][col] >= heights[row][col + 1] {
            let mut flag = true;
            for j in col + 1..n {
                if heights[row][col] < heights[row][j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                connect[row][col].1 = true;
            } else {
                if !connect[row][col + 1].1 {
                    // 当前节点的高度大于等于下一列的高度，能不能继续向右走就看下一个节点
                    Self::dfs(heights, connect, row, col + 1, (m, n));
                }
                // 向右走
                connect[row][col].1 = connect[row][col + 1].1;
            }
        }
        if !connect[row][col].0 {
            let mut flag = true;
            for i in (0..row).rev() {
                if heights[row][col] <= heights[i][col] {
                    flag = false;
                    break;
                }
            }
            if flag {
                connect[row][col].0 = true;
            } else {
                match row.checked_sub(1) {
                    Some(up) => {
                        if heights[row][col] >= heights[up][col] {
                            if !connect[up][col].0 {
                                // 当前节点的高度大于等于上一行的高度，能不能继续向上走就看上一个节点
                                Self::dfs(heights, connect, up, col, (m, n));
                            }
                            // 向上走
                            connect[row][col].0 = connect[up][col].0;
                        } else {
                            // 不能向上走
                            connect[row][col].0 = false;
                        }
                    }
                    None => {}
                }
            }
        }
        if !connect[row][col].0 {
            let mut flag = true;
            for j in (0..col).rev() {
                if heights[row][col] <= heights[row][j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                connect[row][col].0 = true;
            } else {
                match col.checked_sub(1) {
                    Some(left) => {
                        if heights[row][col] >= heights[row][left] {
                            if !connect[row][left].0 {
                                // 当前节点的高度大于等于上一列的高度，能不能继续向左走就看上一个节点
                                Self::dfs(heights, connect, row, left, (m, n));
                            }
                            connect[row][col].0 = connect[row][left].0;
                        } else {
                            // 不能向左走
                            connect[row][col].0 = false;
                        }
                    }
                    None => {}
                }
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
