use std::collections::HashSet;
///
/// [1584. 连接所有点的最小费用](https://leetcode.cn/problems/min-cost-to-connect-all-points/?envType=problem-list-v2&envId=minimum-spanning-tree)
///
struct Solution;
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut len = points.len();
        let mut distances = vec![vec![-1; len]; len];
        // 计算任意两点之间的距离
        for i in 0..len {
            for j in 0..=i {
                let val = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                distances[i][j] = val;
                distances[j][i] = val;
            }
        }

        // 记录当前节点到树中所有节点的最短距离
        let mut min_dis = vec![i32::MAX; len];

        // 已加入的树节点
        let mut tree = HashSet::new();

        for i in 0..len {
            if tree.is_empty() {
                tree.insert(0);
                for j in 0..len {
                    let dis = distances[i][j];
                    min_dis[j] = min_dis[j].min(dis);
                }
            } else {
                let mut min_v = i32::MAX;
                let mut min_k = usize::MAX;

                for k in 0..len {
                    if tree.contains(&k) {
                        continue;
                    }
                    if min_dis[k] <= min_v {
                        min_v = min_dis[k];
                        min_k = k;
                    }
                }
                tree.insert(min_k);
                for j in 0..len {
                    if tree.contains(&j) {
                        continue;
                    }
                    for node in &tree {
                        let dis = distances[*node][j];
                        min_dis[j] = min_dis[j].min(dis);
                    }
                }
            }
        }
        min_dis.iter().sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![2, 2],
                vec![3, 10],
                vec![5, 2],
                vec![7, 0]
            ]),
            20
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::min_cost_connect_points(vec![vec![3, 12], vec![-2, 5], vec![-4, 1],]),
            18
        );
    }
}
