///
/// [38. 外观数列](https://leetcode.cn/problems/count-and-say/)
///
struct Solution;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let chs = Self::count_and_say(n - 1).chars().collect::<Vec<char>>();
        let mut ans = String::new();
        let mut last_ch_count = 1;
        let mut last_ch = chs[0];
        for i in 1..chs.len() {
            if chs[i] == last_ch {
                last_ch_count += 1;
            } else {
                ans.push_str(&last_ch_count.to_string());
                ans.push(last_ch);
                last_ch_count = 1;
            }
            last_ch = chs[i];
        }
        ans.push_str(&last_ch_count.to_string());
        ans.push(last_ch);

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::count_and_say(1), "1");
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::count_and_say(2), "11");
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::count_and_say(3), "21");
    }
    #[test]
    fn t4() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }

    #[test]
    fn t5() {
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
