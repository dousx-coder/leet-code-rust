///
/// [96. 不同的二叉搜索树](https://leetcode.cn/problems/unique-binary-search-trees/description/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            // dp[i]是所有[1,i]的情况总数和

            // 以j为根节点的情况：
            // 左子树[1,j-1]的个数 * 右子树[j+1,i]的个数(笛卡尔积)
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::num_trees(3), 5);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::num_trees(1), 1);
    }
}
