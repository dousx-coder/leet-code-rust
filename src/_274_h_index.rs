///
/// [274. H指数](https://leetcode.cn/problems/h-index/)
///
struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        if citations.len() == 1 {
            return if citations[0] > 0 { 1 } else { 0 };
        }

        let mut citations = citations;
        // 升序排序
        citations.sort_unstable();

        let len = citations.len();
        for i in 0..len {
            let h = (len - i) as i32;
            if citations[i] >= h {
                // 当前引用数 >= 剩余论文数
                return h;
            }
        }
        0
    }

    fn h_index_iter(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        // 降序排序
        citations.sort_unstable_by(|a, b| b.cmp(a));
        citations
            .iter()
            .enumerate()
            .find(|&(i, &citation)| citation <= i as i32) //找到第一个 citation <= index 的位置
            .map_or(citations.len() as i32, |(i, _)| i as i32) // 如果没有找到（所有论文都满足），返回论文总数
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        // 总共有 5 篇论文，每篇论文相应的被引用了 3, 0, 6, 1, 5 次。
        // 由于研究者有 3 篇论文每篇 至少 被引用了 3 次，其余两篇论文每篇被引用 不多于 3 次，所以她的 h 指数是 3。
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(Solution::h_index_iter(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
        assert_eq!(Solution::h_index_iter(vec![1, 3, 1]), 1);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::h_index(vec![100]), 1);
        assert_eq!(Solution::h_index_iter(vec![100]), 1);
    }
}
