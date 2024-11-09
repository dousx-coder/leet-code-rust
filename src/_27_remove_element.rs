///
/// 27.remove_element
///
/// https://leetcode.cn/problems/remove-element/
///

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        nums.retain(|&x| x != val);
        nums.len() as i32

    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let mut p = vec![1, 3, 4, 2];
        let r = Solution::remove_element(&mut p, 3);
        assert_eq!(3, r);
        assert_eq!(vec![1, 4, 2], p)
    }
    #[test]
    fn t2() {
        let mut p = vec![3, 2, 2, 3];
        let r = Solution::remove_element(&mut p, 3);
        assert_eq!(2, r);
        assert_eq!(vec![2, 2], p)
    }
    #[test]
    fn t3() {
        let mut p = vec![3, 3];
        let r = Solution::remove_element(&mut p, 3);
        assert_eq!(0, r);
        assert_eq!(0, p.len());
    }

    #[test]
    fn t4() {
        let mut p = vec![4, 5];
        let r = Solution::remove_element(&mut p, 4);
        assert_eq!(1, r);
        assert_eq!(vec![5], p);
    }
}
