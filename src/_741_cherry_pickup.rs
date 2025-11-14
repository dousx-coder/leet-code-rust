///
/// [741. 摘樱桃](https://leetcode.cn/problems/cherry-pickup/)
///
struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        // 看成两个人 A 和 B，都从 (0,0) 走到右下角 (n−1,n−1)
        let mut memo = vec![vec![vec![-1; n]; n]; n * 2 - 1];
        0.max(Solution::dfs(n * 2 - 2, n - 1, n - 1, &grid, &mut memo))
    }
    fn dfs(
        t: usize,
        j: usize,
        k: usize,
        grid: &Vec<Vec<i32>>,
        meno: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if t < j || t < k || grid[t - j][j] < 0 || grid[t - k][k] < 0 {
            return i32::MIN;
        };
        if t == 0 {
            return grid[0][0];
        }
        if meno[t][j][k] != -1 {
            // 已经计算过
            return meno[t][j][k];
        }

        let a = Self::dfs(t - 1, j, k, grid, meno);
        let b = if let Some(k) = k.checked_sub(1) {
            Self::dfs(t - 1, j, k, grid, meno)
        } else {
            i32::MIN
        };
        let c = if let Some(j) = j.checked_sub(1) {
            Self::dfs(t - 1, j, k, grid, meno)
        } else {
            i32::MIN
        };
        let d = if j >= 1 && k >= 1 {
            Self::dfs(t - 1, j - 1, k - 1, grid, meno)
        } else {
            i32::MIN
        };
        let res = a.max(b).max(c).max(d) + grid[t - j][j] + if k != j { grid[t - k][k] } else { 0 };
        // 记忆搜索
        meno[t][j][k] = res;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::cherry_pickup(vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]]),
            5
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::cherry_pickup(vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]]),
            0
        )
    }

    #[test]
    fn t3() {
        let vec = vec![
            vec![1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::cherry_pickup(vec), 15);
    }
}
