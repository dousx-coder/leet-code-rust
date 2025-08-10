///
/// [475. 供暖器](https://leetcode.cn/problems/heaters/?envType=problem-list-v2&envId=binary-search)
///
struct Solution;
impl Solution {
    /// 当前半径是否可以让供暖器覆盖所有屋子
    fn is_full_coverage(houses: &Vec<i32>, heaters: &Vec<i32>, r: i32) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < houses.len() && j < heaters.len() {
            if (heaters[j] - houses[i]).abs() <= r {
                // 当前屋子在供暖器的半径范围内，i+1 判断下一个屋子
                i += 1;
            } else {
                // 当前屋子不在供暖器的半径范围内，j+1 继续判断下一个供暖器
                j += 1;
            }
        }
        // 所有屋子是否在供暖器的半径范围内
        i >= houses.len()
    }
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut heaters = heaters;
        heaters.sort();
        let mut houses = houses;
        houses.sort();
        // 题目给的范围是1 <= houses[i], heaters[i] <= 10^9
        let mut left = 0;
        let mut right = houses.last().unwrap().max(heaters.last().unwrap()) + 1;
        while left < right {
            let r = left + ((right - left) >> 1);
            if Self::is_full_coverage(&houses, &heaters, r) {
                right = r;
            } else {
                left = r + 1;
            }
        }
        left
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![2]), 1);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::find_radius(vec![1, 5], vec![2]), 3);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::find_radius(vec![1, 5], vec![10]), 9);
    }
    #[test]
    fn t5() {
        assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![1]), 2);
    }
}
