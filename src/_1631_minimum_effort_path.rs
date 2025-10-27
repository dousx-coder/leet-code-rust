use std::cmp::Reverse;
use std::collections::BinaryHeap;

///
/// [1631. 最小体力消耗路径](https://leetcode.cn/problems/path-with-minimum-effort/)
///
struct Solution;
impl Solution {
    /// 你准备参加一场远足活动。给你一个二维 `rows x columns` 的地图 `heights` ，
    /// 其中 `heights[row][col]` 表示格子 `(row, col)` 的高度。一开始你在最左上角的格子 `(0, 0)` ，
    /// 且你希望去最右下角的格子 `(rows-1, columns-1)` （注意下标从 0 开始编号）。
    /// 你每次可以往 上，下，左，右 四个方向之一移动，你想要找到耗费 体力 最小的一条路径。
    ///
    /// 一条路径耗费的 体力值 是路径上相邻格子之间 高度差绝对值 的 最大值 决定的。
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        // 「Dijkstra」
        let (m, n) = (heights.len(), heights[0].len());
        if m * n == 1 {
            // 特殊情况：只有1个格子，那么耗费体力为0
            return 0;
        }
        let mut dist = vec![vec![i32::MAX; n]; m];
        let mut heap = BinaryHeap::new();
        // (距离,row,col) 小顶堆
        heap.push(Reverse((0, 0, 0)));
        let m = m as i32;
        let n = n as i32;
        // 上下左右四个方向
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some(Reverse((d, i, j))) = heap.pop() {
            for (dx, dy) in &dirs {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx < 0 || nx >= m || ny < 0 || ny >= n {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                let max_d = d.max((heights[nx][ny] - heights[i][j]).abs());

                if dist[nx][ny] > max_d {
                    dist[nx][ny] = max_d;
                    heap.push(Reverse((max_d, nx, ny)));
                }
            }
        }
        dist[(m - 1) as usize][(n - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // [1,2,2]
        // [3,8,2]
        // [5,3,5]
        // 路径 [1,3,5,3,5] 连续格子的差值绝对值最大为 2 。
        // 路径 [1,2,2,2,5] 连续格子的差值绝对值最大为 3 。
        // 所以最优的路径是 [1,3,5,3,5] 结果返回最大差值的绝对值2
        assert_eq!(
            Solution::minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]),
            2
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]),
            1
        );
    }
    #[test]
    fn t3() {
        assert_eq!(
            Solution::minimum_effort_path(vec![
                vec![1, 2, 1, 1, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 1, 1, 2, 1]
            ]),
            0
        );
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::minimum_effort_path(vec![vec![3]]), 0);
    }

    #[test]
    fn t5() {
        assert_eq!(
            Solution::minimum_effort_path(vec![vec![1, 10, 6, 7, 9, 10, 4, 9]]),
            9
        );
    }
}
