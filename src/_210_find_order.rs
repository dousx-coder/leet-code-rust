use core::num;

///
/// [210. 课程表Ⅱ](https://leetcode.cn/problems/course-schedule-ii/)
///
struct Solution;
impl Solution {
    /// `prerequisites[i] = [ai, bi]` ，表示在选修课程 `ai` 前 必须 先选修 `bi`
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        // 邻接表
        let mut adj = vec![vec![]; num_courses];
        // 入度数组
        let mut in_degree = vec![0; num_courses];

        for pre in prerequisites {
            let a = pre[0] as usize;
            let b = pre[1] as usize;
            adj[b].push(a);
            in_degree[a] += 1;
        }
        if !in_degree.iter().any(|&x| x == 0) {
            // 不存在入度为0的点，说明有环，不能完成课程
            return vec![];
        }
        let mut ans = vec![];

        let mut queue = std::collections::VecDeque::new();
        for i in 0..num_courses {
            if in_degree[i] == 0 {
                // 入度为0的点入队
                queue.push_back(i);
                ans.push(i as i32);
            }
        }
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    // 入度为0的点入队
                    queue.push_back(v);
                    ans.push(v as i32);
                }
            }
        }
        if queue.is_empty() && in_degree.iter().all(|&x| x == 0) {
            ans
        } else {
            vec![]
        }
    }
}
#[cfg(test)]
mod tests {
    use maplit::hashset;

    use super::*;

    #[test]
    fn t1() {
        let order = Solution::find_order(2, vec![vec![1, 0]]);
        let expected = hashset! {
            vec![0, 1],
            vec![1, 0]
        };
        assert!(expected.contains(&order))
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::find_order(1, vec![]), vec![0]);
    }
}
