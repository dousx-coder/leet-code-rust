use std::collections::{HashSet, VecDeque};

///
/// [547. 省份的数量](https://leetcode.cn/problems/number-of-provinces/?envType=problem-list-v2&envId=depth-first-search)
///
struct Solution;
impl Solution {
    /// `isConnected`是 `n x n` 的矩阵
    ///
    /// 其中 `isConnected[i][j] = 1` 表示第 `i` 个城市和第 `j` 个城市直接相连
    ///
    /// 而 `isConnected[i][j] = 0` 表示二者不直接相连
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        Self::recursion(is_connected)
    }
    /// 集合判断解法
    pub fn set(is_connected: Vec<Vec<i32>>) -> i32 {
        // 数组是左下和右上是对称的i与j相连，则isConnected[i][j]==isConnected[j][i]
        // 所以只需要遍历一半
        let each_vec = || -> Vec<HashSet<usize>> {
            let n = is_connected.len();

            let mut records: Vec<HashSet<usize>> = vec![];
            for i in 0..n {
                for j in 0..=i {
                    if is_connected[i][j] == 1 {
                        if let Some(c) = records
                            .iter_mut()
                            .find(|set| set.contains(&i) || set.contains(&j))
                        {
                            // 这样判断最后需要合并集合
                            // 如果集合a和b相连 b和c相连，如果先遇到a和c，那会把a和c当成新的集合
                            c.insert(i);
                            c.insert(j);
                        } else {
                            // 没有找到包含i或j的集合
                            let mut set = HashSet::new();
                            set.insert(i);
                            set.insert(j);
                            records.push(set);
                        }
                    }
                }
            }

            records
        };

        let mut records = each_vec();
        if records.len() <= 1 {
            return records.len() as i32;
        }
        // 合并所有连通的集合
        // 返回连通集合的数量
        let closure = || -> i32 {
            let records_len = records.len();
            // 创建一个图，其中每个节点指向与其有交集的其他节点
            let mut graph: Vec<Vec<usize>> = vec![Vec::new(); records_len];

            // 构建图：如果两个集合有交集，则在图中添加一条边
            for i in 0..records_len {
                for j in i + 1..records_len {
                    if !records[i].is_disjoint(&records[j]) {
                        graph[i].push(j);
                        graph[j].push(i);
                    }
                }
            }

            let mut visited = vec![false; records_len];
            let mut counter = 0;

            // 遍历每个节点，使用 BFS 或 DFS 找到所有连通分量
            for i in 0..records_len {
                if !visited[i] {
                    let mut component: HashSet<usize> = HashSet::new();
                    let mut queue = VecDeque::new();
                    queue.push_back(i);
                    visited[i] = true;

                    while let Some(node) = queue.pop_front() {
                        // 将当前集合的所有元素合并到当前连通分量中
                        component.extend(&records[node]);
                        // 遍历当前节点的所有邻居
                        for &neighbor in &graph[node] {
                            if !visited[neighbor] {
                                visited[neighbor] = true;
                                queue.push_back(neighbor);
                            }
                        }
                    }
                    counter += 1
                }
            }
            counter
        };
        closure()
    }

    /// 递归解法
    fn recursion(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut count = 0;

        fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
            visited[i] = true;
            for j in 0..is_connected.len() {
                if is_connected[i][j] == 1 && !visited[j] {
                    dfs(is_connected, visited, j);
                }
            }
        }

        for i in 0..n {
            if !visited[i] {
                dfs(&is_connected, &mut visited, i);
                count += 1;
            }
        }

        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::set(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::recursion(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::find_circle_num(vec![
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 1]
            ]),
            1
        );
    }
}
