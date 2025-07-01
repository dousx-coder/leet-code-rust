/// 69 x的平方根
///
/// https://leetcode.cn/problems/sqrtx/?envType=problem-list-v2&envId=binary-search
struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 0 {
            panic!("x < 0");
        }
        let (mut left, mut right) = (0, x);
        while left <= right {
            let mid = (left + right) / 2;
            // 使用 checked_mul 避免整数溢出
            match mid.checked_mul(mid) {
                Some(square) => {
                    if square == x {
                        return mid;
                    } else if square < x {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
                None => {
                    // mid*mid 溢出，说明 mid 太大
                    right = mid - 1;
                }
            }
        }
        // 循环结束时，right 是最接近且不大于 x 的平方根
        right
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
