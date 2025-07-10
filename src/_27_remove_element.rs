///
/// [27.移除元素](https://leetcode.cn/problems/remove-element/)
///
struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        // 快慢指针
        let mut fast = 0;
        let mut slow = 0;
        while fast < nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
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

        assert_eq!(vec![1, 4, 2], &p[0..3])
    }
    #[test]
    fn t2() {
        let mut p = vec![3, 2, 2, 3];
        let r = Solution::remove_element(&mut p, 3);
        assert_eq!(2, r);
        assert_eq!(vec![2, 2], &p[0..2])
    }
    #[test]
    fn t3() {
        let mut p = vec![3, 3];
        let r = Solution::remove_element(&mut p, 3);
        assert_eq!(0, r);
    }

    #[test]
    fn t4() {
        let mut p = vec![4, 5];
        let r = Solution::remove_element(&mut p, 4);
        assert_eq!(1, r);
        assert_eq!(vec![5], &p[0..1]);
    }
}
