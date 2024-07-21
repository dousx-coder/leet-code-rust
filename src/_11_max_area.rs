#[allow(dead_code)]
struct Solution {}

///
/// 11. 盛最多水的容器
///
/// https://leetcode.cn/problems/container-with-most-water/description/
///

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut area = 0;
        while left <= right {
            let width = (right - left) as i32;
            let left_v = height.get(left).unwrap();
            let right_v = height.get(right).unwrap();
            let temp_area = if left_v < right_v {
                left += 1;
                (left_v * width) as i32
            } else {
                right -= 1;
                (right_v * width) as i32
            };
            if temp_area > area {
                area = temp_area;
            }
            if left >= height.len() || right <= 0 {
                break;
            }
        }
        area
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let r = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(r, 49);
    }
    #[test]
    fn test2() {
        let r = Solution::max_area(vec![1, 1]);
        assert_eq!(r, 1);
    }
}
