use std::collections::HashMap;


struct Solution {}

// #[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(&x) = map.get(&(target - num)) {
                return vec![x, i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // nums = [3,3], target = 6
        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
