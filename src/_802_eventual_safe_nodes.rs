///
/// [802. 找到最终的安全状态](https://leetcode.cn/problems/find-eventual-safe-states/?envType=problem-list-v2&envId=topological-sort)
///
struct Solution;
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        // 构建反向图和入度数组
        let mut reverse_graph = vec![Vec::new(); n];
        let mut in_degree = vec![0; n];

        for (i, neighbors) in graph.iter().enumerate() {
            in_degree[i] = neighbors.len();
            for &neighbor in neighbors {
                reverse_graph[neighbor as usize].push(i);
            }
        }

        // 将所有终端节点加入队列
        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        // 拓扑排序
        let mut safe = vec![false; n];
        while let Some(node) = queue.pop_front() {
            // 标记为安全节点
            safe[node] = true;
            for &prev in &reverse_graph[node] {
                in_degree[prev] -= 1;
                if in_degree[prev] == 0 {
                    // 如果入度为0，说明该节点的所有后继都是安全的
                    queue.push_back(prev);
                }
            }
        }

        // 收集结果
        (0..n as i32).filter(|&i| safe[i as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![]
            ]),
            vec![2, 4, 5, 6]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![1, 2, 3, 4],
                vec![1, 2],
                vec![3, 4],
                vec![0, 4],
                vec![]
            ]),
            vec![4]
        )
    }

    #[test]
    fn t3() {
        // 0 空
        // 1-->0  1-->2 1-->3 1-->4
        // 2-->3
        // 3-->4
        // 4 空
        assert_eq!(
            Solution::eventual_safe_nodes(vec![vec![], vec![0, 2, 3, 4], vec![3], vec![4], vec![]]),
            vec![0, 1, 2, 3, 4]
        )
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![4, 9],
                vec![3, 5, 7],
                vec![0, 3, 4, 5, 6, 8],
                vec![7, 8, 9],
                vec![5, 6, 7, 8],
                vec![6, 7, 8, 9],
                vec![7, 9],
                vec![8, 9],
                vec![9],
                vec![]
            ]),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        )
    }
}
