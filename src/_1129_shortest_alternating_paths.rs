use std::collections::VecDeque;

///
/// [1129. 颜色交替的最短路径](https://leetcode.cn/problems/shortest-path-with-alternating-colors/?envType=problem-list-v2&envId=graph)
///
struct Solution;
impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;

        // 构建邻接表
        let mut graph_red = vec![vec![]; n];
        let mut graph_blue = vec![vec![]; n];

        for edge in red_edges {
            graph_red[edge[0] as usize].push(edge[1] as usize);
        }

        for edge in blue_edges {
            graph_blue[edge[0] as usize].push(edge[1] as usize);
        }

        // BFS搜索，获取从节点0开始到各节点的最短交替路径
        let mut answer = vec![-1; n];
        let mut queue = VecDeque::new();

        // (node, color) -> color: 0表示红边，1表示蓝边
        queue.push_back((0, 0)); // 从节点0开始，下一条边是红边
        queue.push_back((0, 1)); // 从节点0开始，下一条边是蓝边

        answer[0] = 0;

        // visited[node][color] 表示通过color颜色的边到达node节点是否已访问
        let mut visited = vec![vec![false; 2]; n];
        visited[0][0] = true;
        visited[0][1] = true;

        let mut dist = 0;

        while !queue.is_empty() {
            dist += 1;
            let size = queue.len();

            for _ in 0..size {
                let (node, color) = queue.pop_front().unwrap();
                // 切换颜色
                let next_color = 1 - color;
                // 根据下一条边的颜色选择对应的图
                let graph = if next_color == 0 {
                    &graph_red
                } else {
                    &graph_blue
                };

                // 遍历当前节点通过next_color颜色的边能到达的所有节点
                for &next_node in &graph[node] {
                    // 如果该节点通过next_color颜色的边未被访问过
                    if !visited[next_node][next_color] {
                        visited[next_node][next_color] = true;
                        // 如果是第一次到达该节点，则记录距离(dist是递增的，所以这里记录的就是最短距离)
                        if answer[next_node] == -1 {
                            answer[next_node] = dist;
                        }
                        queue.push_back((next_node, next_color));
                    }
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]),
            vec![0, 1, -1]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]),
            vec![0, 1, -1]
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::shortest_alternating_paths(
                5,
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
                vec![vec![1, 2], vec![2, 3], vec![3, 1]]
            ),
            vec![0, 1, 2, 3, 7]
        );
    }
}
