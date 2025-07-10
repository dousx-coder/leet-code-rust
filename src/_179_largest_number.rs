///
/// [179. 最大的数](https://leetcode.cn/problems/largest-number/?envType=problem-list-v2&envId=greedy)
///
struct Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| {
            let a = a.to_string();
            let b = b.to_string();
            let a = a + &b;
            let b = b + &a;
            b.cmp(&a)
        });
        if nums[0] == 0 {
            "0".to_string()
        } else {
            nums.iter().map(|&x| x.to_string()).collect::<String>()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::largest_number(vec![0, 0, 0]), "0".to_string());
    }
}
