/// 435 无重叠区间
///
/// https://leetcode.cn/problems/non-overlapping-intervals/
struct Solution;
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            // left升序
            let ordering = a[0].cmp(&b[0]);
            // 如果left相同，则根据right升序
            // 尽量将跨度长的区间移除这样，移除的总数才能达到最小
            // 排序前： [[1, 2], [2, 3], [3, 4], [1, 3]]
            // 排序后： [[1, 2], [1, 3], [2, 3], [3, 4]]
            ordering.then(a[1].cmp(&b[1]))
        });
        let mut erase = 0;
        let mut right = intervals[0][1];
        for i in 1..intervals.len() {
            let interval = &intervals[i];
            if interval[0] < right {
                erase += 1;
                // 更新right为当前区间的右边界
                right = right.min(interval[1]);
            } else {
                // 更新right
                right = interval[1];
            }
        }
        erase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 5], vec![1, 2]]),
            2
        );
    }
    #[test]
    fn t3() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![-52, 31],
                vec![-73, -26],
                vec![82, 97],
                vec![-65, -11],
                vec![-62, -49],
                vec![95, 99],
                vec![58, 95],
                vec![-31, 49],
                vec![66, 98],
                vec![-63, 2],
                vec![30, 47],
                vec![-40, -26]
            ]),
            7
        );
    }
}
