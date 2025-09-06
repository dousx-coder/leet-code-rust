///
///  [565.数组嵌套](https://leetcode.cn/problems/array-nesting/?envType=problem-list-v2&envId=depth-first-search)
///
struct Solution;
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        // 实际上是需要找到数组中存在的对大环的长度
        let n = nums.len();
        let half = (n / 2) as i32;
        let mut max_count = 0;
        let mut visited = vec![false; n];
        for i in 0..n {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            let mut count = 1;
            let mut j = nums[i] as usize;
            while nums[j] != nums[i] {
                visited[j] = true;
                j = nums[j] as usize;
                count += 1;
            }
            if count > half {
                return count;
            }
            max_count = max_count.max(count);
        }
        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // S[0] = {A[0], A[A[0]], A[A[A[0]]], A[A[A[A[0]]]]} = {5, 6, 2, 0}
        // S[0] = {A[0], A[5], A[6], A[2]} = {5, 6, 2, 0}
        let nums = vec![5, 4, 0, 3, 1, 6, 2];
        let res = Solution::array_nesting(nums);
        assert_eq!(res, 4);
    }

    #[test]
    fn t2() {
        let nums = vec![0, 1, 2];
        let res = Solution::array_nesting(nums);
        assert_eq!(res, 1);
    }

    #[test]
    fn t3() {
        let ans = Solution::array_nesting(vec![1, 2, 0]);
        assert_eq!(ans, 3);
    }
}
