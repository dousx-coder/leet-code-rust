/// 967 连续差相同的数字
///
/// https://leetcode.cn/problems/numbers-with-same-consecutive-differences/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        Self::backtracking(n, k, &mut vec![], &mut ans);
        ans
    }
    fn backtracking(n: i32, k: i32, curr: &mut Vec<i32>, ans: &mut Vec<i32>) {
        if curr.len() == n as usize {
            let s = curr.iter().map(|x| x.to_string()).collect::<String>();
            let num = s.parse::<i32>().unwrap();
            ans.push(num);
            return;
        }
        let start = if curr.len() == 0 { 1 } else { 0 };
        for i in start..10 {
            if curr.is_empty() || (*curr.last().unwrap() - i).abs() == k {
                curr.push(i);
                Solution::backtracking(n, k, curr, ans);
                curr.pop();
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;
    #[test]
    fn t1() {
        let expected = hashset![181, 292, 707, 818, 929];
        let ans = Solution::nums_same_consec_diff(3, 7);
        assert_eq!(expected.len(), ans.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }

    #[test]
    fn t2() {
        let expected = hashset![
            10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98
        ];
        let ans = Solution::nums_same_consec_diff(2, 1);
        assert_eq!(expected.len(), ans.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }

    #[test]
    fn t3() {
        let expected = hashset![11, 22, 33, 44, 55, 66, 77, 88, 99];
        let ans = Solution::nums_same_consec_diff(2, 0);
        assert_eq!(expected.len(), ans.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }

    #[test]
    fn t4() {
        let expected = hashset![13, 20, 24, 31, 35, 42, 46, 53, 57, 64, 68, 75, 79, 86, 97];
        let ans = Solution::nums_same_consec_diff(2, 2);
        assert_eq!(expected.len(), ans.len());
        assert_eq!(HashSet::from_iter(ans.into_iter()), expected);
    }
}
