use itertools::Itertools;
///
/// [556. 下一个更大元素 III](https://leetcode.cn/problems/next-greater-element-iii/)
///
struct Solution;
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        if n < 10 {
            return -1;
        }
        let n = n as i64;
        let mut nums = n.to_string().chars().collect::<Vec<char>>();
        let len = nums.len();
        for i in (0..len).rev() {
            for j in (i + 1..len).rev() {
                if nums[i] < nums[j] {
                    nums.swap(i, j);
                    nums[i + 1..].reverse();
                    return Self::ans(n, &mut nums).unwrap_or_else(|value| value);
                }
            }
        }
        nums.reserve(nums.len());
        Self::ans(n, &mut nums).unwrap_or_else(|value| value)
    }

    fn ans(n: i64, nums: &mut Vec<char>) -> Result<i32, i32> {
        let ans = nums.iter().join("").parse::<i64>().unwrap();
        if ans > i32::MAX as i64 {
            return Err(-1);
        }
        Ok(if ans == n { -1 } else { ans as i32 })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::next_greater_element(12), 21);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::next_greater_element(21), -1);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::next_greater_element(2147483486), -1);
    }
}
