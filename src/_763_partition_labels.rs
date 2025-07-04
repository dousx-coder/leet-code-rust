///
/// 793 划分字母区间
///
/// https://leetcode.cn/problems/partition-labels/
struct Solution;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let chs = s.chars().collect::<Vec<char>>();
        let mut last = vec![-1; 26];
        let a_ascii = 'a' as usize;
        // 预计算每个字符最后一次出现的位置
        for (i, ch) in chs.iter().enumerate() {
            let idx = (*ch) as usize - a_ascii;
            last[idx] = i as i32;
        }
        let mut ch_max = 0;
        let mut i = 0;
        let len = chs.len();
        let mut ans = vec![];
        while i < len {
            let chs_i = chs[i] as usize - a_ascii;
            ch_max = ch_max.max(last[chs_i]);
            let mut j = i + 1;
            while j <= ch_max as usize {
                let chs_j = chs[j] as usize - a_ascii;
                ch_max = ch_max.max(last[chs_j]);
                j += 1;
            }
            ans.push((j - i) as i32);
            i = j;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_string()),
            vec![10]
        );
    }
}
