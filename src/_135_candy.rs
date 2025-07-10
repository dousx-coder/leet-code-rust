///
/// [135. 分发糖果](https://leetcode.cn/problems/candy/)
///
struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        // 初始化数字每人至少1个
        let mut candies = vec![1; len];
        for i in 0..len {
            if i > 0 && ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (0..len).rev() {
            if i < len - 1 && ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }
        candies.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }

    #[test]
    fn t3() {
        // [1, 2, 1, 2, 1, 1, 1]
        // [1, 2, 1, 4, 3, 2, 1]
        assert_eq!(Solution::candy(vec![1, 2, 2, 5, 4, 3, 2]), 14);
    }
}
