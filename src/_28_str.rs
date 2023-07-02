struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let f = haystack.find(&needle);
        if f.is_some() {
            return f.unwrap() as i32;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test1() {
        assert_eq!(Solution::str_str(String::from("sadbutsad"), String::from("sad")), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::str_str(String::from("leetcode"), String::from("leeto")), -1);
    }
}