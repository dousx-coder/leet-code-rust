///
/// [611. 有效三角形个数](https://leetcode.cn/problems/valid-triangle-number/?envType=problem-list-v2&envId=sorting)
///
struct Solution;
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let len = nums.len();
        let mut count = 0;
        for k in 2..len {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    // 固定大边，找到符合条件的两条小边
                    // 因为是排序后的数字，所以从i到j-1的数字 加上j和k对应的两个数字 都符合条件
                    count += j - i;

                    // 移动j，即减小右指针
                    j -= 1;
                } else {
                    // 移动i，即增大左指针
                    i += 1;
                }
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        let nums = vec![2, 2, 4, 3];
        let res = Solution::triangle_number(nums);
        assert_eq!(res, 3);
    }

    #[test]
    fn t2() {
        let nums = vec![4, 2, 3, 4];
        let res = Solution::triangle_number(nums);
        assert_eq!(res, 4);
    }
}
