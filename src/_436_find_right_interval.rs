///
/// [436. 寻找右区间](https://leetcode.cn/problems/find-right-interval/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let len = intervals.len();
        let mut ans = vec![-1; len];

        // 创建包含起始点和原始索引的向量，并按起始点排序
        let mut intervals_with_index: Vec<_> = intervals
            .iter()
            .enumerate()
            .map(|(i, interval)| (interval[0], i))
            .collect();

        intervals_with_index.sort_by_key(|&(start, _)| start);

        // 对每个区间，在排序后的数组中二分查找最右边的区间
        for (i, interval) in intervals.iter().enumerate() {
            let end = interval[1];

            // 使用二分查找找到第一个起始点 >= end 的区间
            match intervals_with_index.binary_search_by_key(&end, |&(start, _)| start) {
                Ok(pos) => {
                    // 找到精确匹配，直接使用该索引
                    ans[i] = intervals_with_index[pos].1 as i32;
                }
                Err(pos) => {
                    // 没有精确匹配，pos 是第一个大于 end 的位置
                    if pos < intervals_with_index.len() {
                        ans[i] = intervals_with_index[pos].1 as i32;
                    }
                }
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
