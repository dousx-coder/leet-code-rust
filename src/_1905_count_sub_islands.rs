///
/// [1905. 统计子岛屿](https://leetcode.cn/problems/count-sub-islands/)
///
struct Solution;
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let m = grid1.len();
        let n = grid1[0].len();
        let mut grid2 = grid2;
        let mut counter = 0;

        for i in 0..m {
            for j in 0..n {
                // 如果在grid2中找到一个岛屿点，进行DFS检查是否为子岛屿
                if grid2[i][j] == 1 && Self::is_sub_island(m, n, &grid1, &mut grid2, i, j) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn is_sub_island(
        m: usize,
        n: usize,
        grid1: &Vec<Vec<i32>>,
        grid2: &mut Vec<Vec<i32>>,
        i: usize,
        j: usize,
    ) -> bool {
        // 边界检查
        if i >= m || j >= n {
            return true;
        }

        // 如果当前位置在grid2中不是陆地，则不影响是否为子岛屿
        if grid2[i][j] != 1 {
            return true;
        }

        // 标记当前位置已访问
        grid2[i][j] = 2;

        // 检查当前位置在grid1中是否也是陆地
        let is_current_sub = grid1[i][j] == 1;

        // 递归检查四个方向，必须所有部分都是子岛屿
        let is_down_sub = Self::is_sub_island(m, n, grid1, grid2, i + 1, j);
        let is_right_sub = Self::is_sub_island(m, n, grid1, grid2, i, j + 1);
        let is_up_sub = match i.checked_sub(1) {
            Some(x) => Self::is_sub_island(m, n, grid1, grid2, x, j),
            None => true,
        };
        let is_left_sub = match j.checked_sub(1) {
            Some(y) => Self::is_sub_island(m, n, grid1, grid2, i, y),
            None => true,
        };

        // 当前岛屿是子岛屿当且仅当当前位置在grid1中是陆地，且所有相邻部分也是子岛屿
        is_current_sub && is_down_sub && is_right_sub && is_up_sub && is_left_sub
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        let grid1 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
        ];
        let grid2 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 1],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];
        assert_eq!(Solution::count_sub_islands(grid1, grid2), 3);
    }

    #[test]
    fn t2() {
        let grid1 = vec![
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
        ];
        let grid2 = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 0, 0, 1],
        ];
        assert_eq!(Solution::count_sub_islands(grid1, grid2), 2);
    }
}
