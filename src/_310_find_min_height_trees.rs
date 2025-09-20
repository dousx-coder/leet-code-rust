///
/// [310. 最小高度树](https://leetcode.cn/problems/minimum-height-trees/?envType=problem-list-v2&envId=graph)
///
struct Solution;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if edges.is_empty() {
            return vec![0];
        }
        let n = n as usize;
        // 临界表
        let mut graph = vec![vec![]; n];
        // 逐步删除入度为1的点
        edges.iter().for_each(|edge| {
            // 创建邻接表
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            if !graph[u].contains(&v) {
                // 测试参数中有重复的输入
                graph[u].push(v);
            }
            if !graph[v].contains(&u) {
                graph[v].push(u);
            }
        });
        while graph.iter().filter(|v| v.len() >= 1).count() >= 2 {
            match Self::is_find(&graph, n) {
                Some(res) => {
                    return res;
                }
                None => {}
            }
            let mut retain = vec![];
            for i in 0..n {
                if graph[i].len() == 1 {
                    let u = graph[i][0];
                    if !graph[u].is_empty() {
                        // graph[u].retain(|&x| x != i);
                    }
                    retain.push((u, i));
                    graph[i].clear();
                }
            }

            match Self::is_find(&graph, n) {
                Some(res) => {
                    return res;
                }
                None => {}
            }

            retain.iter().for_each(|&(u, v)| {
                graph[u].retain(|&x| x != v);
            });

            match Self::is_find(&graph, n) {
                Some(res) => {
                    return res;
                }
                None => {}
            }
        }
        println!("{:?}", graph);
        graph
            .iter()
            .enumerate()
            .filter(|(_, v)| !v.is_empty())
            .map(|(i, _)| i as i32)
            .collect()
    }

    fn is_find(graph: &Vec<Vec<usize>>, n: usize) -> Option<Vec<i32>> {
        let mut res = vec![];
        for j in 0..n {
            if !graph[j].is_empty() {
                res.push(j);
            }
        }
        if res.len() <= 2 {
            Some(res.iter().map(|&x| x as i32).collect())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            ),
            vec![3, 4]
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::find_min_height_trees(3, vec![vec![1, 0], vec![1, 2], vec![2, 1]]),
            vec![1]
        );
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::find_min_height_trees(2, vec![vec![1, 0]]),
            vec![0, 1]
        );
    }
}
