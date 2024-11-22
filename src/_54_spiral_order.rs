use std::cmp::min;

///
/// https://leetcode.cn/problems/spiral-matrix/
///
struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // 1 2 3 4
        // 5 6 7 8
        // 9 10 11 12
        // 行 3
        let row = matrix.len();
        // 列 4
        let col = matrix[0].len();
        // 数量
        let num_total = row * col;
        // 第几圈
        let mut circle = 1;
        let mut ans = vec![];
        while ans.len() < num_total {
            // 采用左闭右开 每条边上最后一个节点 作为下一次的起始点
            // 向右  (0,0) (0,1) (0,2)
            for j in (circle - 1)..=(col - circle) {
                ans.push(matrix[circle - 1][j]);
            }

            // 行列不相等 遍历次数不一定是4的倍数，例如3行4列 最后一次遍历只遍历了向右
            if ans.len() == num_total {
                break;
            }

            // 向下 (0,3) (1,3)
            for i in circle..=(row - circle) {
                ans.push(matrix[i][col - circle]);
            }

            if ans.len() == num_total {
                break;
            }

            // 向左 (2,3)  (2,2) (2,1)
            for i in ((circle - 1)..=(col - circle - 1)).rev() {
                ans.push(matrix[row - circle][i]);
            }
            if ans.len() == num_total {
                break;
            }

            // 向上 (2,0) (1,0)
            for i in (circle..=(row - circle - 1)).rev() {
                ans.push(matrix[i][circle - 1]);
            }
            circle += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let vec = Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(vec, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn t2() {
        let vec = Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]);
        assert_eq!(vec, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}
