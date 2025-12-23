use std::collections::BinaryHeap;
///
/// [215. 数组中第K个大的元素](https://leetcode.cn/problems/kth-largest-element-in-an-array/?envType=problem-list-v2&envId=quickselect)
///
struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(num);
        }
        for _ in 1..k {
            heap.pop();
        }

        match heap.peek() {
            Some(ans) => *ans,
            None => panic!("no answer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
