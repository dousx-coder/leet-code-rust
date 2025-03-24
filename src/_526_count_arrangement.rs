/// 526 优美的排列
///
/// https://leetcode.cn/problems/beautiful-arrangement/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut nums: Vec<usize> = vec![];
        let mut used = vec![false; (n + 1) as usize];
        nums.push(0);
        for i in 1..=n {
            nums.push(i as usize);
        }
        let path = &mut vec![];
        // 不使用下标0
        path.push(0);
        let mut ans = 0;
        Self::backtracking(&mut ans, path, &nums, &mut used);
        ans
    }
    fn backtracking(ans: &mut i32, path: &mut Vec<usize>, nums: &Vec<usize>, used: &mut Vec<bool>) {
        if path.len() == nums.len() {
            *ans += 1;
            return;
        }
        for i in 1..nums.len() {
            if used[i] {
                continue;
            }
            let len = path.len();
            if nums[i] % len != 0 && len % nums[i] != 0 {
                // 剪枝：如果当前数字不满足条件，直接跳过(这里是path.len)
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            Self::backtracking(ans, path, nums, used);
            path.pop();
            used[i] = false;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::count_arrangement(1), 1);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::count_arrangement(2), 2);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::count_arrangement(3), 3);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::count_arrangement(12), 4010);
    }
}
