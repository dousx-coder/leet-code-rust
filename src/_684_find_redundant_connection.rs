use std::collections::{HashSet, VecDeque};
///
/// [684. 冗余连接](https://leetcode.cn/problems/redundant-connection/description/?envType=problem-list-v2&envId=graph)
///
struct Solution;
impl Solution {
    /// 树可以看成是一个连通且 无环 的 无向 图。
    ///
    /// 给定一个图，该图从一棵 `n` 个节点 (节点值 `1～n`) 的树中添加一条边后获得。添加的边的两个不同顶点编号在 `1` 到 `n` 中间，
    /// 且这条附加的边不属于树中已存在的边。图的信息记录于长度为 `n` 的二维数组 `edges` ，`edges[i] = [ai, bi]` 表示图中在 `ai` 和 `bi` 之间存在一条边。
    ///
    /// 请找出一条可以删去的边，删除后可使得剩余部分是一个有着 `n` 个节点的树。如果有多个答案，则返回数组 `edges` 中最后出现的那个。
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::union_find(edges)
    }

    fn topological_sort(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![vec![]; edges.len() + 1];

        let mut degree = vec![0; edges.len() + 1];
        for edge in edges.iter() {
            let mut a = edge[0] as usize;
            let mut b = edge[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
            degree[a] += 1;
            degree[b] += 1;
        }

        let mut queue = VecDeque::new();
        for i in 1..=edges.len() {
            if (&graph[i]).len() == 1 {
                queue.push_back(i);
            }
        }
        // 记录图中所有可以被删除的节点，也就是那些通过拓扑排序算法识别出来的、不属于环的节点
        let mut ring = HashSet::new();
        while let Some(node) = queue.pop_front() {
            ring.insert(node as i32);
            for &next in graph[node].iter() {
                degree[next] -= 1;
                if degree[next] == 1 {
                    // 度数为1的点表示是叶子节点
                    queue.push_back(next);
                }
            }
        }
        edges
            .iter()
            .rev()
            .find(|edge| !ring.contains(&edge[0]) && !ring.contains(&edge[1]))
            .unwrap()
            .to_vec()
    }

    /// 并查集解题
    fn union_find(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut len = edges.len() + 1;
        let mut father = vec![0; len];
        for i in 0..len {
            father[i] = i;
        }
        for (_, ele) in edges.iter().enumerate() {
            let u = ele[0] as usize;
            let v = ele[1] as usize;
            if Self::find(&mut father, u) != Self::find(&mut father, v) {
                Self::join(&mut father, u, v);
            } else {
                return ele.to_vec();
            }
        }
        vec![]
    }
    fn join(father: &mut Vec<usize>, x: usize, y: usize) {
        let (x, y) = (Self::find(father, x), Self::find(father, y));
        father[x] = y;
    }

    fn find(father: &mut Vec<usize>, x: usize) -> usize {
        if father[x] != x {
            father[x] = Self::find(father, father[x]);
        }
        father[x]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
    }

    #[test]
    fn t2() {
        // 2 - 1 - 5
        // |   |
        // 3 - 4
        assert_eq!(
            Solution::topological_sort(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::union_find(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
