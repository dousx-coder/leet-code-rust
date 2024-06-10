#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();
        let len = str.len();
        let strv = str.as_bytes().to_vec();
        let mut index = 0;
        while index < len / 2 {
            let s1 = strv.get(index);
            let s2 = strv.get(len - 1 - index);
            if s1 != s2 {
                return false;
            }
            index += 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(false, Solution::is_palindrome(14));
    }

    #[test]
    fn t2() {
        assert_eq!(true, Solution::is_palindrome(141));
    }

    #[test]
    fn t3() {
        assert_eq!(false, Solution::is_palindrome(142));
    }

    #[test]
    fn t4() {
        assert_eq!(true, Solution::is_palindrome(1441));
    }
}

