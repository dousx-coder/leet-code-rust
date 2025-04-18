/// 406 根据身高重构队列
///
/// https://leetcode.cn/problems/queue-reconstruction-by-height/
struct Solution;
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        // 根据身高降序排序，当身高相同时，按 k 升序排序
        people.sort_by(|a, b| {
            if a[0] == b[0] {
                // 当身高相同时，按 k 升序排序
                a[1].cmp(&b[1])
            } else {
                // 按身高降序排序
                b[0].cmp(&a[0])
            }
        });
        let mut result = Vec::with_capacity(people.len());
        for person in people {
            // 根据 k 插入
            result.insert(person[1] as usize, person);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let ans = Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ]);
        assert_eq!(
            ans,
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }
}
