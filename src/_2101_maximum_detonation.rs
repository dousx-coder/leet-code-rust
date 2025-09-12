///
/// [2101. 引爆最多的炸弹](https://leetcode.cn/problems/detonate-the-maximum-bombs/)
///
struct Solution;
impl Solution {
    /// 炸弹用一个下标从 0 开始的二维整数数组 `bombs` 表示，其中 `bombs[i] = [xi, yi, ri]` 。`xi` 和 `yi` 表示第 `i` 个炸弹的 `X` 和 `Y` 坐标，`ri` 表示爆炸范围的半径。
    ///
    ///你需要选择引爆 一个 炸弹。当这个炸弹被引爆时，所有 在它爆炸范围内的炸弹都会被引爆，这些炸弹会进一步将它们爆炸范围内的其他炸弹引爆。
    ///
    ///给你数组 `bombs` ，请你返回在引爆 一个 炸弹的前提下，最多 能引爆的炸弹数目。
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        // dfs 从每个不同的炸弹开始引爆，求最大值
        let mut max = 0;
        for (i, bomb) in bombs.iter().enumerate() {
            let mut visited = vec![false; bombs.len()];
            let bomb = Self::dfs(&bombs, &mut visited, i);
            max = max.max(bomb);
        }
        max
    }

    fn dfs(bombs: &Vec<Vec<i32>>, visited: &mut Vec<bool>, index: usize) -> i32 {
        if visited[index] {
            // 已经被引爆，不重复计数
            return 0;
        }
        // 标记当前炸弹已被引爆
        visited[index] = true;
        let mut count = 1;

        // 遍历所有炸弹，判断是否在当前炸弹的爆炸范围内
        for (i, bomb) in bombs.iter().enumerate() {
            if i != index && !visited[i] && Self::overlap(&bombs[index], bomb) {
                count += Self::dfs(bombs, visited, i);
            }
        }
        count
    }
    /// 判断两个炸弹的爆炸范围是否重叠
    ///
    /// 取值范围： `1 <= xi, yi, ri <= 10^5`，这里转成`i64`避免`i32`溢出
    fn overlap(bomb1: &Vec<i32>, bomb2: &Vec<i32>) -> bool {
        (bomb1[2] as i64 + bomb2[2] as i64).pow(2)
            >= (bomb1[0] as i64 - bomb2[0] as i64).pow(2)
                + (bomb1[1] as i64 - bomb2[1] as i64).pow(2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]]),
            2
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::maximum_detonation(vec![vec![1, 1, 5], vec![10, 10, 5]]),
            1
        );
    }
    #[test]
    fn t3() {
        assert_eq!(
            Solution::maximum_detonation(vec![
                vec![1, 2, 3],
                vec![2, 3, 1],
                vec![3, 4, 2],
                vec![4, 5, 3],
                vec![5, 6, 4]
            ]),
            5
        );
    }
    #[test]
    fn t4() {
        assert_eq!(
            Solution::maximum_detonation(vec![vec![1, 1, 100000], vec![100000, 100000, 1]]),
            1
        );
    }
    #[test]
    fn t5() {
        assert_eq!(
            Solution::maximum_detonation(vec![
                vec![54, 95, 4],
                vec![99, 46, 3],
                vec![29, 21, 3],
                vec![96, 72, 8],
                vec![49, 43, 3],
                vec![11, 20, 3],
                vec![2, 57, 1],
                vec![69, 51, 7],
                vec![97, 1, 10],
                vec![85, 45, 2],
                vec![38, 47, 1],
                vec![83, 75, 3],
                vec![65, 59, 3],
                vec![33, 4, 1],
                vec![32, 10, 2],
                vec![20, 97, 8],
                vec![35, 37, 3]
            ]),
            1
        );
    }
}
