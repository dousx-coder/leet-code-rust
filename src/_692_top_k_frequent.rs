/// 692 前K个高频单词
///
/// https://leetcode.cn/problems/top-k-frequent-words/?envType=problem-list-v2&envId=counting
struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        // 统计每个单词的出现次数
        let mut counts = HashMap::new();
        for word in words {
            *counts.entry(word).or_insert(0) += 1;
        }

        // 转换为 Vec<(String, u32)> 以便排序
        let mut entries: Vec<(String, u32)> = counts.into_iter().collect();

        // 排序
        entries.sort_by(|a, b| {
            if a.1 == b.1 {
                // 相同频率按字典序升序
                a.0.cmp(&b.0)
            } else {
                // 不同频率按频率降序
                b.1.cmp(&a.1)
            }
        });

        // 取前 k 个单词
        entries
            .into_iter()
            .take(k as usize)
            .map(|(word, _)| word)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn t1() {
        let words = vec!["i", "love", "leetcode", "i", "love", "coding"]
            .iter()
            .map(|x| x.to_string())
            .collect_vec();
        assert_eq!(Solution::top_k_frequent(words, 2), vec!["i", "love"]);
    }

    #[test]
    fn t2() {
        let words = vec![
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect_vec();
        assert_eq!(
            Solution::top_k_frequent(words, 4),
            vec!["the", "is", "sunny", "day"]
        );
    }
}
