use std::collections::HashMap;

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
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试函数
    #[test]
    fn test1() {
        // nums = [3,3], target = 6
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
    }
}
