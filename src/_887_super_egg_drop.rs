use std::collections::HashMap;

///
/// [887. 鸡蛋掉落](https://leetcode.cn/problems/super-egg-drop/description/)
///
struct Solution;
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::dp(k, n, &mut memo)
    }
    fn dp(k: i32, n: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(&res) = memo.get(&(k, n)) {
            return res;
        }

        let res = if n == 0 {
            0
        } else if k == 1 {
            n
        } else {
            let (mut low, mut high) = (1, n);
            let mut min_moves = n;

            while low <= high {
                let mid = (low + high) / 2;
                let broken = Self::dp(k - 1, mid - 1, memo);
                let not_broken = Self::dp(k, n - mid, memo);

                if broken > not_broken {
                    high = mid - 1;
                    min_moves = min_moves.min(broken + 1);
                } else {
                    low = mid + 1;
                    min_moves = min_moves.min(not_broken + 1);
                }
            }

            min_moves
        };

        memo.insert((k, n), res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::super_egg_drop(1, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::super_egg_drop(2, 6);
        assert_eq!(result, 3);
    }
    #[test]
    fn t3() {
        let result = Solution::super_egg_drop(3, 14);
        assert_eq!(result, 4);
    }
}
