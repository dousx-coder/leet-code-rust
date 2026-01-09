///
/// [67. 二进制求和](https://leetcode.cn/problems/add-binary/?envType=problem-list-v2&envId=simulation)
///
struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.chars().rev().collect::<Vec<char>>();
        let b = b.chars().rev().collect::<Vec<char>>();
        let a_len = a.len();
        let b_len = b.len();
        let max_len = a_len.max(b_len);
        let mut carry = 0;
        let mut res = vec![];
        for i in 0..max_len {
            let a_val = if i < a_len {
                // 要么是0要么是1，不用转十进制，直接if判断会快一点
                if a[i] == '0' { 0 } else { 1 }
            } else {
                0
            };
            let b_val = if i < b_len {
                if b[i] == '0' { 0 } else { 1 }
            } else {
                0
            };

            let sum = a_val + b_val + carry;

            carry = sum / 2;
            if sum % 2 == 1 {
                res.insert(0, '1');
            } else {
                res.insert(0, '0');
            }
        }
        if carry == 1 {
            res.insert(0, '1');
        }
        String::from_iter(res.iter())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::add_binary(String::from("11"), String::from("1")),
            "100"
        );
    }

    #[test]
    fn t2() {
        //  1010
        // +1011
        // 10101
        assert_eq!(
            Solution::add_binary(String::from("1010"), String::from("1011")),
            "10101"
        );
    }
}
