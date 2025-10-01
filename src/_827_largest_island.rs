use std::collections::{HashMap, HashSet};

///
/// [827. 最大人工岛](https://leetcode.cn/problems/making-a-large-island/)
///
struct Solution;
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![0; n]; m];
        let mut mark = 2;
        // 0和1 下标不用
        let mut lands = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && visited[i][j] == 0 {
                    let area =
                        Solution::find_land_boundary(m, n, &mut grid, &mut visited, mark, i, j);
                    if area > 0 {
                        lands.insert(mark, area);
                        mark += 1;
                    }
                }
            }
        }
        let mut max = *lands.values().max().unwrap_or(&0);
        // 特殊情况：所有格子都是陆地
        if max == (m * n) as i32 {
            return max;
        }
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let join = Self::adjoin_area(i, j, &visited, &lands, m, n) + 1;
                    max = max.max(join);
                }
            }
        }
        return max;
    }

    /// 获取四个方向的陆地面积总和
    fn adjoin_area(
        i: usize,
        j: usize,
        visited: &Vec<Vec<i32>>,
        lands: &HashMap<i32, i32>,
        m: usize,
        n: usize,
    ) -> i32 {
        let mut marks = HashSet::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (di, dj) in directions.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let (ni, nj) = (ni as usize, nj as usize);
                let mark = visited[ni][nj];
                if mark > 0 && lands.contains_key(&mark) {
                    marks.insert(mark);
                }
            }
        }

        marks.iter().map(|&mark| lands[&mark]).sum()
    }
    /// 查找陆地面积，同时将陆地标记为mark
    fn find_land_boundary(
        m: usize,
        n: usize,
        grid: &mut Vec<Vec<i32>>,
        visited: &mut Vec<Vec<i32>>,
        mark: i32,
        i: usize,
        j: usize,
    ) -> i32 {
        if i >= m || j >= n || visited[i][j] != 0 || grid[i][j] == 0 {
            return 0;
        }

        visited[i][j] = mark;
        let mut area = 1;

        // 四个方向递归搜索
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (di, dj) in directions.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni >= 0 && nj >= 0 {
                let (ni, nj) = (ni as usize, nj as usize);
                area += Self::find_land_boundary(m, n, grid, visited, mark, ni, nj);
            }
        }

        area
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::largest_island(vec![vec![1, 1], vec![1, 0]]), 4);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::largest_island(vec![vec![1, 1], vec![1, 1]]), 4);
    }
}
