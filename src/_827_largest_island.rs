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
        let mut lands = HashMap::new();
        let mut mark = 2;
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
        let mut max = lands.values().cloned().max().unwrap_or(0);
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
        // 记录mark序号，避免重复累加
        let mut marks = HashSet::new();

        if let Some(nexti) = i.checked_sub(1) {
            if visited[nexti][j] > 0 {
                let mark = visited[nexti][j];
                if lands.get(&mark).is_some() {
                    marks.insert(mark);
                };
            }
        }
        if let Some(nexti) = i.checked_add(1) {
            if nexti < m && visited[nexti][j] > 0 {
                let mark = visited[nexti][j];
                if lands.get(&mark).is_some() {
                    marks.insert(mark);
                };
            }
        }
        if let Some(nextj) = j.checked_sub(1) {
            if visited[i][nextj] > 0 {
                let mark = visited[i][nextj];
                if lands.get(&mark).is_some() {
                    marks.insert(mark);
                };
            }
        }
        if let Some(nextj) = j.checked_add(1) {
            if nextj < n && visited[i][nextj] > 0 {
                let mark = visited[i][nextj];
                if lands.get(&mark).is_some() {
                    marks.insert(mark);
                };
            }
        }
        if marks.is_empty() {
            return 0;
        }

        marks.iter().map(|mark| lands.get(mark).unwrap_or(&0)).sum()
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

        area += Self::find_land_boundary(m, n, grid, visited, mark, i + 1, j);

        if let Some(nexti) = i.checked_sub(1) {
            area += Self::find_land_boundary(m, n, grid, visited, mark, nexti, j);
        }

        area += Self::find_land_boundary(m, n, grid, visited, mark, i, j + 1);

        if let Some(nextj) = j.checked_sub(1) {
            area += Self::find_land_boundary(m, n, grid, visited, mark, i, nextj);
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
