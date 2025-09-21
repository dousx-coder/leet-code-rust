///
/// [LCR 108. 单词接龙](https://leetcode.cn/problems/om3reC/)
///
struct Solution;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // 记录起始节点到当前节点的最短路径
        let mut begin_to_curr_map = std::collections::HashMap::new();
        let mut deque = std::collections::VecDeque::new();
        deque.push_back(begin_word.clone());
        begin_to_curr_map.insert(begin_word.clone(), 1);
        while let Some(curr) = deque.pop_front() {
            for word in word_list.iter() {
                // 找到差异只有1个字符的
                if word.len() != curr.len()
                    || word
                        .chars()
                        .zip(curr.chars())
                        .filter(|(a, b)| a != b)
                        .count()
                        > 1
                {
                    continue;
                }
                if *word != end_word && !begin_to_curr_map.contains_key(word) {
                    deque.push_back(word.clone());
                    begin_to_curr_map.insert(word.clone(), begin_to_curr_map[&curr] + 1);
                } else if *word == end_word {
                    return begin_to_curr_map[&curr] + 1;
                }
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            5
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string()
                ]
            ),
            0
        );
    }
}
