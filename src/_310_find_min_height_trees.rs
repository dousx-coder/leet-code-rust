///
/// [310. 最小高度树](https://leetcode.cn/problems/minimum-height-trees/?envType=problem-list-v2&envId=graph)
///
struct Solution;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let n = n as usize;
        // 构建邻接表和度数组
        let mut graph = vec![vec![]; n];
        let mut degree = vec![0; n];

        // 初始化图和度数
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            if !graph[u].contains(&v) {
                graph[u].push(v);
                degree[u] += 1;
            }
            if !graph[v].contains(&u) {
                graph[v].push(u);
                degree[v] += 1;
            }
        }

        // 找到所有叶子节点（度为1的节点）
        let mut leaves = Vec::new();
        for i in 0..n {
            if degree[i] == 1 {
                leaves.push(i);
            }
        }

        let mut remaining_nodes = n;

        // 从外向内逐层删除叶子节点
        while remaining_nodes > 2 {
            let leaves_count = leaves.len();
            remaining_nodes -= leaves_count;

            // 存储下一层的叶子节点
            let mut new_leaves = Vec::new();

            // 删除当前所有叶子节点
            for leaf in leaves {
                // 对于每个叶子节点，找到其相邻节点
                for &neighbor in &graph[leaf] {
                    degree[neighbor] -= 1;
                    // 如果相邻节点变成叶子节点，加入下一轮处理
                    if degree[neighbor] == 1 {
                        new_leaves.push(neighbor);
                    }
                }
            }

            leaves = new_leaves;
        }

        // 剩下的节点就是最小高度树的根节点
        leaves.into_iter().map(|x| x as i32).collect()
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
