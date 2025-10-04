///
/// [685. 冗余连接Ⅱ](https://leetcode.cn/problems/redundant-connection-ii/)
///
struct Solution;
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let edges: Vec<_> = edges
            .iter()
            .map(|edge| vec![edge[0] as usize, edge[1] as usize])
            .collect();
        let mut parent = vec![0; n + 1];
        let mut cand_a = vec![];
        let mut cand_b = vec![];

        // 检查是否存在入度为2的节点
        for edge in &edges {
            let u = edge[0];
            let v = edge[1];
            if parent[v] == 0 {
                parent[v] = u;
            } else {
                // 节点 v 已经有父节点（v节点入度≥ 2）
                cand_a = vec![parent[v], v];
                cand_b = vec![u, v];
                // 此处并未更新 parent[v] 的值，以此来模拟删除 cand_b 的操作。
            }
        }

        // 初始化并查集路径
        let mut parent = (0..=n).collect::<Vec<_>>();
        if cand_b.is_empty() {
            // 如果没有任何节点的入度≥2，那就找出形成环路的那条边即可
            let mut parent = (0..=n).collect::<Vec<_>>();
            for edge in &edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                if !Self::union(u, v, &mut parent) {
                    return edge.iter().map(|&x| x as i32).collect::<Vec<i32>>();
                }
            }
            panic!()
        } else {
            // 存在入度≥2的节点
            for edge in &edges {
                if edge == &cand_b {
                    continue;
                }

                let u = edge[0] as usize;
                let v = edge[1] as usize;
                if !Self::union(u, v, &mut parent) {
                    return cand_a.iter().map(|&x| x as i32).collect::<Vec<i32>>();
                }
            }
            cand_b.iter().map(|&x| x as i32).collect::<Vec<i32>>()
        }
    }

    fn find(x: usize, parent: &mut Vec<usize>) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent[x], parent);
        }
        parent[x]
    }

    fn union(u: usize, v: usize, parent: &mut Vec<usize>) -> bool {
        let root_u = Self::find(u, parent);
        let root_v = Self::find(v, parent);
        if root_u == root_v {
            return false;
        }
        parent[root_v] = root_u;
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::find_redundant_directed_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 1],
                vec![1, 5]
            ]),
            vec![4, 1]
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::find_redundant_directed_connection(vec![
                vec![1, 2],
                vec![4, 1],
                vec![2, 4],
                vec![3, 2]
            ]),
            vec![1, 2]
        );
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::find_redundant_directed_connection(vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 5]
            ]),
            vec![2, 3]
        );
    }
}
