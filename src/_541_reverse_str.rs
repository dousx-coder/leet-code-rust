struct Solution;
impl Solution {
    fn swap(x: &mut [char], left: usize, right: usize) {
        let (mut left, mut right) = (left, right - 1);
        while left < right {
            x.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    pub fn reverse_str(s: String, k: i32) -> String {
        let mut vec = s.chars().collect::<Vec<char>>();
        let k = k as usize;
        let _2k = 2 * k;
        for i in (0..vec.len()).step_by(_2k) {
            if i + _2k <= vec.len() {
                let right = i + k;
                let mut x = &mut vec[i..right];
                Solution::swap(&mut x, 0, right - i);
                continue;
            }
            if i + k > vec.len() {
                // 剩余的不到K
                let left = i;
                let right = s.len();
                let mut x = &mut vec[left..];
                Solution::swap(&mut x, left - i, right - i);
                continue;
            }
            // 剩余部分超过K 不足2K
            let left = i;
            let right = i + k;
            let mut x = &mut vec[left..right];
            Solution::swap(&mut x, left - i, right - i);
            break;
        }
        String::from_iter(vec.iter())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::reverse_str("abcdefg".to_string(), 2),
            "bacdfeg".to_string()
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::reverse_str("abcd".to_string(), 2),
            "bacd".to_string()
        );
    }
}
