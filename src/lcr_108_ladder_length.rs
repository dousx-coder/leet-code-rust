use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
///
/// [LCR 108. 单词接龙](https://leetcode.cn/problems/om3reC/)
///
struct Solution;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // 使用 Rc<String> 替代 String，避免克隆实际字符串数据
        let begin_word_rc = Rc::new(begin_word);
        let end_word_rc = Rc::new(end_word);

        // 将 word_list 转换为 Rc<String> 向量
        let word_list_rc: Vec<Rc<String>> = word_list.into_iter().map(Rc::new).collect();

        // 记录起始节点到当前节点的最短路径
        let mut begin_to_curr_map = HashMap::new();
        let mut deque = VecDeque::new();

        deque.push_back(begin_word_rc.clone());
        begin_to_curr_map.insert(begin_word_rc.clone(), 1);

        while let Some(curr) = deque.pop_front() {
            for word_rc in word_list_rc.iter() {
                // 找到差异只有1个字符的
                if word_rc.len() != curr.len()
                    || word_rc
                        .chars()
                        .zip(curr.chars())
                        .filter(|(a, b)| a != b)
                        .count()
                        > 1
                {
                    continue;
                }
                if word_rc.as_str() == end_word_rc.as_str() {
                    return begin_to_curr_map[&curr] + 1;
                }
                if !begin_to_curr_map.contains_key(word_rc) {
                    deque.push_back(word_rc.clone());
                    begin_to_curr_map.insert(word_rc.clone(), begin_to_curr_map[&curr] + 1);
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
