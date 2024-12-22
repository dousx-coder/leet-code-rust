///
///46. 全排列
///
/// https://leetcode.cn/problems/permutations/description/
///

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nc = nums.clone();
        Self::recursion(&mut result, &mut nc, 0, nums.len() - 1);
        result
    }
    fn recursion(result: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, start: usize, end: usize) {
        if start == end {
            result.push(nums.clone());
            return;
        }
        for i in start..=end {
            nums.swap(start, i);
            Self::recursion(result, nums, start + 1, end);
            nums.swap(start, i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let r = Solution::permute(vec![1, 2, 3]);

        assert_eq!(
            r,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2]
            ]
        )
    }
}
