///
/// [324. 摆动排序Ⅱ](https://leetcode.cn/problems/wiggle-sort-ii/?envType=problem-list-v2&envId=sorting)
///
struct Solution;
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort_unstable();
        let n = nums.len();
        let mut right = n - 1;
        let mut left = right / 2;
        let cn = nums.clone();

        for i in 0..n {
            if i % 2 == 0 {
                nums[i] = cn[left];
                match left.checked_sub(1) {
                    Some(l) => {
                        left = l;
                    }
                    None => {}
                }
            } else {
                nums[i] = cn[right];
                match right.checked_sub(1) {
                    Some(r) => {
                        right = r;
                    }
                    None => {}
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        //  1 1 1 4 5 6
        let mut nums = vec![1, 5, 1, 1, 6, 4];
        Solution::wiggle_sort(&mut nums);
        println!("{:?}", nums);
        assert!(vec![1, 6, 1, 5, 1, 4] == nums || vec![1, 4, 1, 5, 1, 6] == nums);
    }

    #[test]
    fn t2() {
        let mut nums = vec![1, 3, 2, 2, 3, 1];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(vec![2, 3, 1, 3, 1, 2], nums);
    }
}
