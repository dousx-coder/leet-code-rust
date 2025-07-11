///
/// [1049. 最后一块石头的重量 II](https://leetcode.cn/problems/last-stone-weight-ii/description/)
///
struct Solution;
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        //  将石头分割成2堆，使得2堆石头的重量尽可能的接近
        // target是向下取整
        let target = (sum / 2) as usize;
        let mut dp = vec![0; target + 1];
        for i in 0..stones.len() {
            let stone = stones[i];
            for j in (stone as usize..=target).rev() {
                dp[j] = dp[j].max(dp[j - stone as usize] + stone);
            }
        }
        // dp[target]表示其中一堆石头的最大重量
        let other = sum - dp[target];
        (other - dp[target]).abs()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    }
}
