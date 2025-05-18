/// 01 背包问题
struct Solution;
impl Solution {
    ///
    /// [`capacity`] 背包容量
    ///
    /// [`goods`] 容量: 0表示物品价值,1表示物品重量
    ///
    pub fn max_price(capacity: usize, goods: Vec<(usize, usize)>) -> usize {
        let goods_num = goods.len();
        // dp[i][j] 表示前i个物品，选取任意数量放入容量j内的最大价值
        let mut dp = vec![vec![0; capacity + 1]; goods_num];
        for i in 0..capacity {
            let first = goods[0];
            let first_wight = first.1;
            if i < first_wight {
                continue;
            }
            let first_price = first.0;
            dp[0][i] = first_price;
        }

        for i in 1..goods_num {
            // 第i个物品
            let article = goods[i];
            let price = article.0;
            let weight = article.1;
            for j in 1..=capacity {
                if j < weight {
                    // 不选当前物品
                    dp[i][j] = dp[i - 1][j];
                } else {
                    // 选当前物品
                    dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - weight] + price);
                }
            }
        }
        dp[goods_num - 1][capacity]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        //             背包容量j
        //        0   1   2    3  4
        // 物品[0] 0   15  15  15  0   
        // 物品[1] 0   15  15  20  35  
        // 物品[2] 0   15  15  20  35  
        let ans = Solution::max_price(4, vec![(15, 1), (20, 3), (30, 4)]);
        assert_eq!(ans, 35);
    }

    #[test]
    fn t2() {
        // 0   0   6   6   6   6   6   6   6   6   0   
        // 0   0   6   6   6   6   6   9   9   9   9   
        // 0   0   6   6   6   6   11  11  11  11  11  
        // 0   0   6   6   10  10  11  11  15  15  15  
        // 0   0   6   6   10  12  12  16  16  17  17  
        let ans = Solution::max_price(10, vec![(6, 2), (3, 5), (5, 4), (4, 2), (6, 3)]);
        assert_eq!(ans, 17);
    }
}
