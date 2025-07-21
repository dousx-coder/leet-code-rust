///
/// [57. 插入区间](https://leetcode.cn/problems/insert-interval/)
///
struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut left = 0;
        let mut right = intervals.len();
        // 二分查找插入节点的位置
        while left < right {
            let mid = left + ((right - left) >> 1);
            if intervals[mid][0] > new_interval[0] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        intervals.insert(left, new_interval);

        let mut ans: Vec<Vec<i32>> = Vec::new();
        // 合并区间
        for i in 0..intervals.len() {
            let mut vec = intervals[i].clone();
            while !ans.is_empty() && ans.last().unwrap()[1] >= vec[0] {
                let pop = ans.pop().unwrap();
                vec = vec![pop[0].min(vec[0]), pop[1].max(vec[1])];
            }
            ans.push(vec);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::insert(
                vec![vec![0, 1], vec![5, 5], vec![6, 7], vec![9, 11]],
                vec![12, 21]
            ),
            vec![
                vec![0, 1],
                vec![5, 5],
                vec![6, 7],
                vec![9, 11],
                vec![12, 21]
            ]
        );
    }
}
