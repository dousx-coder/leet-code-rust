///
/// 189. 轮转数组
///
/// https://leetcode.cn/problems/rotate-array/description/
///

struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len() as i32;
        let offset = k % len;
        if offset == 0 {
            return;
        }
        let mut temp_vec: Vec<i32> = vec![-1; len as usize];
        for i in 0..offset {
            let index = (len - offset + i) as usize;
            temp_vec[i as usize] = nums[index];
        }
        for i in (0..=len - 1 - offset).rev() {
            let index = (i + offset) as usize;
            nums[index] = nums[i as usize];
        }
        for i in 0..offset {
            let index = i as usize;
            nums[index] = temp_vec[index];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let expect = vec![3, 99, -1, -100];
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        println!("{:?}", nums);
        assert_eq!(expect, nums);
    }
}
