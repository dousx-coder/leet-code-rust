///
/// [357. 统计各位数字都不同的数字个数](https://leetcode.cn/problems/count-numbers-with-unique-digits/description/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n <= 0 {
            return 1;
        }
        // 0 <= n <= 8,这里path用vec就够
        let mut path = vec![];
        // ans从1开始是把0算进去了，下面的循环从1开始(即最高位不能为0)
        let mut ans = 1;
        for i in 1..10 {
            ans += 1;
            path.push(i);
            Solution::backtracking(&mut path, n - 1, &mut ans);
            path.pop();
        }
        ans
    }
    fn backtracking(path: &mut Vec<i32>, n: i32, ans: &mut i32) {
        if n <= 0 {
            return;
        }
        for i in 0..10 {
            if path.contains(&i) {
                continue;
            }
            path.push(i);
            *ans += 1;
            Solution::backtracking(path, n - 1, ans);
            path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    }
}
