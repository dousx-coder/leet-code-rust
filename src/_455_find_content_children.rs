///
/// [455. 分发饼干](https://leetcode.cn/problems/assign-cookies/)
///  
struct Solution;
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_by(|a, b| b.cmp(a));
        s.sort_by(|a, b| b.cmp(a));
        let mut gi = 0;
        let mut si = 0;
        while gi < g.len() && si < s.len() {
            if s[si] >= g[gi] {
                // 投喂饼干
                si += 1;
            }
            gi += 1;
        }
        si as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }
}
