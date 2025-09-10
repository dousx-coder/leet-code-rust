use std::f32::consts::E;

///
/// [672. 灯泡开关Ⅱ](https://leetcode.cn/problems/bulb-switcher-ii/?envType=problem-list-v2&envId=breadth-first-search)
///
struct Solution;
impl Solution {
    /// 开关 1 ：反转当前所有灯的状态（即开变为关，关变为开）
    ///
    /// 开关 2 ：反转编号为偶数的灯的状态（即 0, 2, 4, ...）
    ///
    /// 开关 3 ：反转编号为奇数的灯的状态（即 1, 3, ...）
    ///
    /// 开关 4 ：反转编号为 j = 3k + 1 的灯的状态，其中 k = 0, 1, 2, ...（即 1, 4, 7, 10, ...
    ///
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        if n >= 3 && presses >= 3 {
            return 8;
        }
        if n < 3 {
            1 + if presses > 0 { n } else { 0 } + if presses > 1 && n > 1 { 1 } else { 0 }
        } else {
            1 + 3 * presses
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::flip_lights(1, 1), 2);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::flip_lights(2, 1), 3);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::flip_lights(3, 1), 4);
    }
}
