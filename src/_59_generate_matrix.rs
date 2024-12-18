///
/// https://leetcode.cn/problems/spiral-matrix-ii/description/
///
struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let size = n as usize;
        let mut result = vec![vec![-1; size]; size];
        let mut count = 1;
        // 第几圈
        let mut circle = 0;
        loop {
            // 采用左闭右开 每条边上最后一个节点 作为下一次的起始点
            if size < 1 + circle {
                if size % 2 != 0 {
                    // 中心点
                    let center = size / 2;
                    result[center][center] = count;
                }
                break;
            }
            let start1 = circle;
            let end1 = size - 1 - circle;
            for i in start1..end1 {
                // 向右  (0,0) (0,1) (0,2)
                result[start1][i] = count;
                count += 1;
            }
            let start2 = circle;
            let end2 = size - 1 - circle;
            for i in circle..end2 {
                // 向下 (0,3) (1,3) (2,3)
                result[i][end2] = count;
                count += 1;
            }
            let start3 = circle + 1;
            let end3 = size - circle;
            for i in (start3..end3).rev() {
                // 向左 (3,3) (3,2) (3,1)
                result[end3 - 1][i] = count;
                count += 1;
            }
            let start4 = circle + 1;
            let end4 = size - circle;
            for i in (start4..end4).rev() {
                // 向上 (3,0) (2,0) (1,0)
                result[i][start4 - 1] = count;
                count += 1;
            }
            circle += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        // 1 2 3
        // 8 9 4
        // 7 6 5
        let r = Solution::generate_matrix(3);
        assert_eq!(r, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]);
    }

    #[test]
    fn t2() {
        // 1 2 3 4
        // C D E 5
        // B J F 6
        // A 9 8 7
        let r = Solution::generate_matrix(4);
        assert_eq!(
            r,
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }
    #[test]
    fn t3() {
        let r = Solution::generate_matrix(2);
        assert_eq!(r, vec![vec![1, 2], vec![4, 3]]);
    }
    #[test]
    fn t4() {
        let r = Solution::generate_matrix(1);
        assert_eq!(r, vec![vec![1]]);
    }
}
