///
/// [275. H指数Ⅱ](https://leetcode.cn/problems/h-index-ii/)
///
struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // citations是升序数组
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::h_index(vec![1, 2, 100]), 2);
    }
}
