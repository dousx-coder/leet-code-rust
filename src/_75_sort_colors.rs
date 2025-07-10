///
/// [75. 颜色分类](https://leetcode.cn/problems/sort-colors/)
///
struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // nums[0...zero]==0
        let mut zero = 0;
        // nums[zero...n-1]==2
        let mut two = nums.len();
        let mut i = 0;
        while i < two {
            if nums[i] == 0 {
                nums.swap(i, zero);
                i += 1;
                zero += 1;
                continue;
            }
            if nums[i] == 1 {
                i += 1;
                continue;
            }
            if nums[i] == 2 {
                nums.swap(i, two - 1);
                two -= 1;
                continue;
            }
            panic!()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let mut vec = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn t2() {
        let mut vec = vec![2, 0, 1];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0, 1, 2]);
    }
}
