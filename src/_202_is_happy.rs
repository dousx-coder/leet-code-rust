use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut ins = n;
        loop {
            let v = Self::split_bits(ins);
            if v == 1 {
                return true;
            }
            if !set.insert(v) {
                return false;
            };
            ins = v;
        }
    }
    pub fn split_bits(n: i32) -> i32 {
        let mut num = n;
        let mut sum = 0;
        while num > 0 {
            let t = num % 10;
            sum += t * t;
            num = num / 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::is_happy(19);
        assert_eq!(result, true);
    }
}
