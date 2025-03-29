use std::collections::HashMap;
/// 49 字母异位词分组
///
/// https://leetcode.cn/problems/group-anagrams/description/
struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs {
            let mut chs = str.chars().collect::<Vec<char>>();
            chs.sort();
            let key = chs.iter().collect::<String>();
            let option = map.get_mut(&key);
            match option {
                Some(v) => v.push(str),
                None => {
                    let mut v = vec![];
                    v.push(str);
                    map.insert(key, v);
                }
            }
        }
        map.values()
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<String>>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let strs = strs.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let mut ans = Solution::group_anagrams(strs);
        let mut expected = vec![vec!["tan", "nat"], vec!["eat", "tea", "ate"], vec!["bat"]];
        assert_eq!(ans.len(), expected.len());
        // 对 ans 和 xv 进行排序，以便比较
        ans.sort();
        expected.sort();
        ans.iter_mut().for_each(|v| v.sort());
        expected.iter_mut().for_each(|v| v.sort());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test2() {
        let strs = vec![""];
        let strs = strs.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let mut ans = Solution::group_anagrams(strs);
        let mut expected = vec![vec![""]];
        assert_eq!(ans, expected);
    }
}
