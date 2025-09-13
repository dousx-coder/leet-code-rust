///
/// [207. 课程表](https://leetcode.cn/problems/course-schedule/?envType=problem-list-v2&envId=depth-first-search)
///
struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // 拓扑排序
        // 1. 先找入度为0的点
        // 2. 删除入度为0的点，并将其邻接点的入度减1
        // 3. 重复1、2，直到没有入度为0的点
        // 4. 如果还有点没有被删除，说明有环，不能完成课程

        // 邻接表
        let mut adj = vec![vec![]; num_courses as usize];
        // 入度数组
        let mut in_degree = vec![0; num_courses as usize];
        for pre in prerequisites {
            let u = pre[1] as usize;
            let v = pre[0] as usize;
            adj[u].push(v);
            in_degree[v] += 1;
        }
        if !in_degree.iter().any(|&x| x == 0) {
            // 不存在入度为0的点，说明有环，不能完成课程
            return false;
        }
        let mut queue = std::collections::VecDeque::new();
        for i in 0..num_courses as usize {
            if in_degree[i] == 0 {
                // 入度为0的点入队
                queue.push_back(i);
            }
        }

        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        queue.is_empty() && in_degree.iter().all(|&x| x == 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }
    #[test]
    fn t3() {
        assert_eq!(
            Solution::can_finish(
                20,
                vec![
                    vec![0, 10],
                    vec![3, 18],
                    vec![5, 5],
                    vec![6, 11],
                    vec![11, 14],
                    vec![13, 1],
                    vec![15, 1],
                    vec![17, 4]
                ]
            ),
            false
        );
    }
}
