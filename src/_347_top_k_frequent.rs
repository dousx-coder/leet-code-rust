use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
///
/// `347. 前 K 个高频元素`
///
/// https://leetcode.cn/problems/top-k-frequent-elements/description/
///
struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = nums.len();
        let mut count_map = HashMap::new();
        for x in nums {
            match count_map.get_mut(&x) {
                None => {
                    let mut nc = NumCount::new(x, 1);
                    count_map.insert(x, nc);
                }
                Some(nc) => {
                    nc.count();
                }
            }
        }
        let mut heap = BinaryHeap::new();
        let k = k as usize;
        for x in count_map.into_iter() {
            let count = x.1;
            heap.push(count);
        }
        let mut result = vec![];
        while let Some(count) = heap.pop() {
            result.push(count.num);
            if result.len() >= k {
                return result;
            }
        }
        panic!("{}", format!("k:{k} > nums.len():{len}"));
    }
}
#[derive(Debug)]
struct NumCount {
    num: i32,
    count: i32,
}
impl NumCount {
    fn new(num: i32, count: i32) -> Self {
        NumCount { num, count }
    }
    fn count(&mut self) {
        self.count += 1
    }
}
// 实现 Ord 和 PartialOrd 来定制排序规则
impl Ord for NumCount {
    fn cmp(&self, other: &Self) -> Ordering {
        // 按照出现的频次排序
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for NumCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NumCount {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl Eq for NumCount {}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let ans_vec = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        let ans_set = HashSet::from_iter(ans_vec);
        assert_eq!(ans_set, hashset![1, 2]);
    }

    #[test]
    fn t2() {
        let ans_vec = Solution::top_k_frequent(vec![1], 1);
        let ans_set = HashSet::from_iter(ans_vec);
        assert_eq!(ans_set, hashset![1]);
    }
}
