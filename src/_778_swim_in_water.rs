use std::cmp::Reverse;
use std::collections::BinaryHeap;

///
/// [778. 水位上升的泳池中游泳](https://leetcode.cn/problems/swim-in-rising-water/description/)
///
struct Solution;
impl Solution {
    /// 在一个 `n x n` 的整数矩阵 `grid` 中，每一个方格的值 `grid[i][j]` 表示位置 `(i, j)` 的平台高度。
    ///
    /// 当开始下雨时，在时间为 `t` 时，水池中的水位为 `t` 。你可以从一个平台游向四周相邻的任意一个平台
    /// 但是前提是此时水位必须同时淹没这两个平台。假定你可以瞬间移动无限距离，也就是默认在方格内部游动是不耗时的。
    /// 当然，在你游泳的时候你必须待在坐标方格里面。
    ///
    /// 你从坐标方格的左上平台 `(0，0)` 出发。返回 你到达坐标方格的右下平台 `(n-1, n-1)` 所需的最少时间 。
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        // 相当于从所有左上到右下的路径中找到一条，使得这条路径上的最大值最小
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        // dist记录的是(0,0)到当前坐标的所有路径中最大值最小的路径中的最大值
        let mut dist = vec![vec![i32::MAX; n]; n];
        dist[0][0] = grid[0][0];
        let mut heap = BinaryHeap::new();
        // (最大值,行,列)
        heap.push(Reverse((dist[0][0], 0, 0)));
        let n = n as i32;

        while let Some(Reverse((v, i, j))) = heap.pop() {
            for (dx, dy) in &dirs {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x < 0 || x >= n || y < 0 || y >= n {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                let t = grid[x][y].max(v);
                if t < dist[x][y] {
                    dist[x][y] = t;
                    heap.push(Reverse((t, x, y)));
                }
            }
        }
        let n = n as usize;
        dist[n - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        // [0,2]
        // [1,3]
        // 时间为0时，你位于坐标方格的位置为 (0, 0)。
        // 此时你不能游向任意方向，因为四个相邻方向平台的高度都大于当前时间为 0 时的水位。
        // 等时间到达 3 时，你才可以游向平台 (1, 1). 因为此时的水位是 3，坐标方格中的平台没有比水位 3 更高的，所以你可以游向坐标方格中的任意位置
        assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::swim_in_water(vec![
                vec![0, 1, 2, 3, 4],
                vec![24, 23, 22, 21, 5],
                vec![12, 13, 14, 15, 16],
                vec![11, 17, 18, 19, 20],
                vec![10, 9, 8, 7, 6]
            ]),
            16
        );
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::swim_in_water(vec![vec![3, 2], vec![0, 1]]), 3);
    }
}
