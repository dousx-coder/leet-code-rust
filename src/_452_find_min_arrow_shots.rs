///
/// [452. 用最少数量的箭引爆气球](https://leetcode.cn/problems/minimum-number-of-arrows-to-burst-balloons/)
///
struct Solution;
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        // 按照气球的右边界升序排列
        points.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut ans = 1;
        // 最优（射出的箭数最小）的方法，使得每一支箭的射出位置都恰好对应着某一个气球的右边界。
        let mut end = points[0][1];
        for point in points {
            let start = point[0];
            if start > end {
                // 如果当前气球的start大于箭的end，则需要再射一发箭
                ans += 1;
                end = point[1]
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        // [[1, 6], [3, 7], [2, 8], [7, 12], [10, 16]]
        let ans = Solution::find_min_arrow_shots(vec![
            vec![10, 16],
            vec![2, 8],
            vec![3, 7],
            vec![1, 6],
            vec![7, 12],
        ]);
        assert_eq!(ans, 2);
    }

    #[test]
    fn t2() {
        let ans =
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
        assert_eq!(ans, 4);
    }
}
