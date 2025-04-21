/// 56合并区间
///
/// https://leetcode.cn/problems/merge-intervals/

struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = intervals.len();
        if len <= 1 {
            return intervals;
        }
        let mut intervals = intervals;

        intervals.sort_by(|a, b| {
            // 升序
            a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))
        });

        let mut ans = vec![];
        let mut left = intervals[0][0];
        let mut right = intervals[0][1];

        for i in 1..len {
            let x0 = intervals[i][0];
            let x1 = intervals[i][1];
            if x0 <= right {
                // 说明和之前的区间能重叠
                right = x1.max(right);
            } else {
                ans.push(vec![left, right]);
                left = x0;
                right = x1;
            }
        }
        ans.push(vec![left, right]);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        )
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![2, 3]]),
            vec![vec![1, 4]]
        )
    }
}
