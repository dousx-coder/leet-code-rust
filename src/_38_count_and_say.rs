///
/// 38 外观数列
///
/// https://leetcode.cn/problems/count-and-say/
struct Solution;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n <= 0 {
            panic!("n<=0")
        }
        if n == 1 {
            return "1".to_string();
        }
        let s = Self::count_and_say(n - 1);
        let chs = s.chars().collect::<Vec<char>>();
        let mut ans = vec![];
        let mut last_ch_count = 0;
        let mut last_ch = ' ';
        for i in 0..chs.len() {
            if chs[i] == last_ch {
                last_ch_count += 1;
            } else {
                if last_ch_count != 0 {
                    ans.push(last_ch_count.to_string());
                    ans.push(last_ch.to_string());
                }
                last_ch_count = 1;
            }
            last_ch = chs[i];
        }
        ans.push(last_ch_count.to_string());
        ans.push(last_ch.to_string());

        ans.iter().map(|s| s.to_string()).collect()
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
        let expected = "12-11".replace("-", "");
        assert_eq!(Solution::count_and_say(4), expected);
    }

    #[test]
    fn t5() {
        let expected = "11-12-21".replace("-", "");
        assert_eq!(Solution::count_and_say(5), expected);
    }
}
