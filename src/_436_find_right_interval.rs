///
/// [436. 寻找右区间](https://leetcode.cn/problems/find-right-interval/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let len = intervals.len();
        let mut ans = vec![-1; len];
        let mut intervals_sort = vec![];
        intervals.iter().enumerate().for_each(|(i, v)| {
            intervals_sort.push((v[0], v[1], i));
        });
        intervals_sort.sort_by(|a, b| a.0.cmp(&b.0));

        for i in 0..len {
            let (start1, end1, index1) = intervals_sort[i];
            if start1 > end1 {
                continue;
            }
            let mut last_idx = -1;
            let mut last_sub = -1;
            for j in i..len {
                let (start2, end2, index2) = intervals_sort[j];
                if end1 > start2 {
                    continue;
                }
                // start <= end
                // 本区间的end <= 下一个区间的start
                if last_idx == -1 {
                    last_sub = start2 - end1;
                    last_idx = index2 as i32;
                } else if last_sub > start2 - end1 {
                    last_sub = start2 - end1;
                    last_idx = index2 as i32
                }
            }
            ans[index1] = last_idx;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::find_right_interval(vec![vec![1, 2]]);
        assert_eq!(vec![-1], ans);
    }
    #[test]
    fn t2() {
        let ans = Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]);
        assert_eq!(vec![-1, 0, 1], ans);
    }

    #[test]
    fn t3() {
        let ans = Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]);
        assert_eq!(vec![-1, 2, -1], ans);
    }

    #[test]
    fn t4() {
        let ans = Solution::find_right_interval(vec![vec![1, 1], vec![3, 4]]);
        assert_eq!(vec![0, -1], ans);
    }
}
