/// 860 柠檬水找零
///
/// https://leetcode.cn/problems/lemonade-change/description/
struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;
        let mut twenty = 0;
        for i in 0..bills.len() {
            let bill = bills[i];
            match bill {
                5 => {
                    five += 1;
                }
                10 => {
                    if five > 0 {
                        five -= 1;
                        ten += 1;
                    } else {
                        return false;
                    }
                }
                20 => {
                    if ten > 0 && five > 0 {
                        ten -= 1;
                        five -= 1;
                        twenty += 1;
                    } else if five >= 3 {
                        five -= 3;
                        twenty += 1;
                    } else {
                        return false;
                    }
                }
                _ => {
                    // bills[i] 不是 5 就是 10 或是 20
                    panic!("invalid parameter value: {bill}")
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 10]), true);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
    }
}
