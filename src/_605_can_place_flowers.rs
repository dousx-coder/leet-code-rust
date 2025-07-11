///
///[605. 种花问题](https://leetcode.cn/problems/can-place-flowers/?envType=problem-list-v2&envId=greedy)
///
struct Solution;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let len = flowerbed.len();

        let mut count = 0;

        if flowerbed[0] == 0 {
            if len > 1 && flowerbed[1] == 1 {
                // 下标0不能种花
            } else {
                // 花坛边缘种花，最多影响右侧1个位置；中间种花两侧都不能种，
                flowerbed[0] = 1;
                count += 1;
            }
        }

        for i in 1..len {
            let pre = flowerbed[i - 1];
            if pre == 1 {
                continue;
            }
            if flowerbed[i] == 1 {
                continue;
            }
            if i + 1 < len && flowerbed[i + 1] == 1 {
                continue;
            }
            flowerbed[i] = 1;
            count += 1;
        }
        count >= n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
            false
        );
    }
    #[test]
    fn t4() {
        assert_eq!(Solution::can_place_flowers(vec![0, 0, 1, 0, 1], 1), true);
    }

    #[test]
    fn t5() {
        assert_eq!(Solution::can_place_flowers(vec![0, 1, 0], 1), false);
    }
}
