///
/// 401 二进制手表
///
/// https://leetcode.cn/problems/binary-watch/description/?envType=problem-list-v2&envId=backtracking
use std::collections::HashSet;


struct Solution;
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on == 0 {
            return vec!["0:00".to_string()];
        }
        let v1 = vec![1, 2, 4, 8];
        let v2 = vec![1, 2, 4, 8, 16, 32];
        let mut ans = vec![];
        for i in 0..4 {
            // 上面最多同时亮3个，如果4个全亮，则是15不符合逻辑,所以i的取值是[0,4)
            let mut hours = vec![];
            Solution::backtracking(0, 11, i, &mut vec![], &mut hours, &v1);
            let hours: Vec<i32> = hours.iter().map(|x| x.iter().sum::<i32>()).collect();
            let j = turned_on - (i as i32);
            if j >= 6 || j < 0 {
                continue;
            }
            // 分钟
            let mut minutes = vec![];
            Solution::backtracking(0, 59, j as usize, &mut vec![], &mut minutes, &v2);
            if minutes.is_empty() {
                continue;
            }
            let minutes: &HashSet<i32> = &minutes.into_iter().map(|x| x.iter().sum()).collect();
            for hour in hours {
                for minute in minutes {
                    ans.push(format!("{}:{:02}", hour, minute));
                }
            }
        }
        ans
    }
    fn backtracking(
        start: usize,
        max_num: i32,
        max_count: usize,
        path: &mut Vec<i32>,
        combination: &mut Vec<Vec<i32>>,
        num: &Vec<i32>,
    ) {
        if path.len() >= max_count {
            combination.push(path.clone());
            return;
        }
        for i in start..num.len() {
            let sum = path.iter().sum::<i32>() + num[i];
            if sum <= max_num {
                // 包含当前数字
                path.push(num[i]);
                Self::backtracking(i + 1, max_num, max_count, path, combination, num);
                path.pop();
            } else {
                Self::backtracking(i + 1, max_num, max_count, path, combination, num);
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    // group_by()
    use itertools::Itertools;
    use maplit::hashset;
    use std::collections::{HashMap, HashSet};

    fn test_backtracking(vec: &Vec<i32>, max: i32) {
        // 组合
        let mut combination = vec![];
        for i in 0..vec.len() {
            Solution::backtracking(0, max, i, &mut vec![], &mut combination, &vec);
        }
        let map = combination
            .into_iter()
            .into_group_map_by(|vec| vec.len()) // 根据向量长度分组
            .into_iter()
            .map(|(len, group)| (len, group.into_iter().collect::<Vec<Vec<i32>>>())) // 将每个组收集到Vec中
            .collect::<HashMap<usize, Vec<Vec<i32>>>>();
        for i in 0..=max as usize {
            if let Some(x) = map.get(&i) {
                println!("Length {}: {:?}", i, x);
            }
        }
    }

    #[test]
    fn t0() {
        test_backtracking(&vec![1, 2, 4, 8], 11);
        test_backtracking(&vec![1, 2, 4, 8, 16, 32], 59);
    }
    #[test]
    fn t1() {
        let expected = hashset! {"0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"}
            .into_iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        let ans = Solution::read_binary_watch(1);
        let set = HashSet::from_iter(ans.into_iter());
        assert_eq!(expected, set);
    }

    #[test]
    fn t2() {
        let ans = Solution::read_binary_watch(9);
        assert!(ans.is_empty());
    }
}
