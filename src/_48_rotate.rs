///
/// [48. 旋转图形](https://leetcode.cn/problems/rotate-image/)
///
struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // 按照对角线翻转，然后按行逆序
        let n = matrix.len();
        for i in 0..n {
            for j in 0..i {
                let mut a = matrix[i][j];
                let mut b = matrix[j][i];
                matrix[i][j] = b;
                matrix[j][i] = a;
            }
        }
        for i in 0..n {
            matrix[i].reverse();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
    #[test]
    fn t2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
