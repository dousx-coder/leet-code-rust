use std::collections::{HashMap, HashSet};
///
/// [229. 多数元素Ⅱ](https://leetcode.cn/problems/majority-element-ii/description/)
///
struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }
        let len = nums.len();
        // 三分之一
        let third = len / 3;
        let mut result = HashSet::new();
        let mut map = HashMap::new();
        for x in nums {
            let option = map.get(&x);
            let count = match option {
                Some(v) => {
                    let c = v + 1;
                    map.insert(x, c);
                    c
                }
                None => {
                    map.insert(x, 1);
                    1
                }
            };
            if count > third {
                result.insert(x);
            }
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    #[test]
    fn t1() {
        let result = Solution::majority_element(vec![3, 2, 3]);
        assert_eq!(result, vec![3]);
    }
    #[test]
    fn t2() {
        let result = Solution::majority_element(vec![1]);
        assert_eq!(result, vec![1]);
    }
    #[test]
    fn t3() {
        let ans = Solution::majority_element(vec![1, 2]);
        let ans: HashSet<_> = ans.into_iter().collect();
        assert_eq!(ans, hashset! {1, 2});
    }
}
