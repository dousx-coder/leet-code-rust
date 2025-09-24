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
        let mut answer = vec![-1; n];
        // 特例
        answer[0] = 0;

        //  入度表
        let mut in_degree_red = vec![vec![]; n];
        let mut in_degree_blue = vec![vec![]; n];
        for edge in red_edges {
            in_degree_red[edge[1] as usize].push(edge[0] as usize);
        }

        for edge in blue_edges {
            in_degree_blue[edge[1] as usize].push(edge[0] as usize);
        }

        // 0到x的最短距离
        for x in 1..n {
            // 必须是红蓝交替的路径
            // 分别以红、蓝两种颜色开始

            // 先找x的入度
            let read_x = &in_degree_red[x];
            let blue_x: &Vec<usize> = &in_degree_blue[x];
            if read_x.is_empty() && blue_x.is_empty() {
                // x不可达
                answer[x] = -1;
                continue;
            }

            let mut dist_red = vec![-1; n];
            let mut dist_blue = vec![-1; n];

            // 用dfs,red和blue交替取值
            let a = Self::bfs(x, 0, &in_degree_red, &in_degree_blue);
            let b = Self::bfs(x, 1, &in_degree_red, &in_degree_blue);
            if a > 0 && b > 0 {
                answer[x] = a.min(b);
            } else if a > 0 && b < 0 {
                answer[x] = a;
            } else if a < 0 && b > 0 {
                answer[x] = b;
            } else {
                answer[x] = -1;
            }
        }

        answer
    }

    /// color: 0-red, 1-blue
    fn bfs(
        x: usize,
        color: i32,
        in_degree_red: &Vec<Vec<usize>>,
        in_degree_blue: &Vec<Vec<usize>>,
    ) -> i32 {
        let mut visited = vec![false; in_degree_blue.len()];
        visited[x] = true;
        let mut color = color % 2;
        let vec = if color == 0 {
            // red
            &in_degree_red[x]
        } else {
            // blue
            &in_degree_blue[x]
        };
        let mut queue = VecDeque::new();
        if vec.is_empty() {
            return -1;
        }
        for &v in vec {
            if v == 0 {
                return 1;
            }
            queue.push_back(v);
            visited[v] = true;
        }

        // 循环需要处理
        let mut path = 1;

        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                color += 1;
                color %= 2;
                let node = queue.pop_front().unwrap();
                let next_vec = if color == 0 {
                    // red
                    &in_degree_red[node]
                } else {
                    // blue
                    &in_degree_blue[node]
                };
                for &v in next_vec {
                    if v == 0 {
                        return path + 1;
                    }
                    if !visited[v] {
                        queue.push_back(v);
                        visited[v] = true;
                    }
                }
            }
            path += 1;
        }
        -1
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
