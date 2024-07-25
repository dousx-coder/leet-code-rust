///
/// 27.remove_element
///
/// https://leetcode.cn/problems/remove-element/
///
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut swap_index = nums.len() - 1;
        for i in 0..=swap_index {
            if i >= nums.len() {
                return nums.len() as i32;
            };
            if i == swap_index || i == nums.len() - 1 {
                return if nums[i] == val {
                    nums.remove(i);
                    i
                } else {
                    i + 1
                } as i32;
            }
            if nums[i] == val {
                while nums[i] == val {
                    if swap_index <= 0 {
                        nums.remove(0);
                        return 0;
                    }
                    nums.swap(i, swap_index);
                    nums.remove(swap_index);
                    if i >= swap_index {
                        return swap_index as i32;
                    }
                    swap_index -= 1;
                }
            }
        }
        swap_index as i32
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
        assert_eq!(vec![1, 2, 4], p)
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
