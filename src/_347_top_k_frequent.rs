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
        // 小顶堆 当长度超过k的时候弹出，最后将
        let k = k as usize;
        let capacity = k + 1;
        let mut min_heap = BinaryHeap::with_capacity(capacity);
        for nc in count_map.values().into_iter() {
            min_heap.push(Reverse(nc));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }
        min_heap.iter().map(|e| e.0.num).collect()
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
