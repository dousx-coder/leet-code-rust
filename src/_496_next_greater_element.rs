use itertools::Itertools;
use std::collections::HashMap;

/// 496 下一个更大的元素Ⅰ
///
/// https://leetcode.cn/problems/next-greater-element-i/
struct Solution;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // stack单调栈
        let mut stack = vec![0];
        // 记录下一个更大元素的下标，-1表示不存在,
        // next_greater_num2_index[j]=k,表示nums2[j]的下一个更大元素是nums2[k]
        let mut next_greater_num2_index = vec![-1; nums2.len()];
        for i in 1..nums2.len() {
            while !stack.is_empty() {
                let last_index = *stack.last().unwrap();
                if nums2[i] > nums2[last_index] {
                    next_greater_num2_index[last_index] = i as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        let mut map = HashMap::new();
        for i in 0..next_greater_num2_index.len() {
            let index = next_greater_num2_index[i];
            let c = if index <= 0 {
                -1
            } else {
                nums2[index as usize]
            };
            map.insert(nums2[i], c);
        }
        nums1
            .iter()
            .map(|x| *map.get(x).unwrap_or(&-1))
            .collect_vec()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
