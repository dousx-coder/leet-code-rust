///
/// [1584. 连接所有点的最小费用](https://leetcode.cn/problems/min-cost-to-connect-all-points/?envType=problem-list-v2&envId=minimum-spanning-tree)
///
struct Solution;
impl Solution {
    /// Prim算法
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 1 {
            return 0;
        }

        // 标记节点是否已在MST中
        let mut in_mst = vec![false; n];

        // 到MST的最小距离
        let mut min_cost = vec![i32::MAX; n];

        min_cost[0] = 0;
        let mut total_cost = 0;

        for _ in 0..n {
            // 找到距离MST最近的未访问节点
            let mut u = 0;
            let mut min_distance = i32::MAX;

            for i in 0..n {
                if !in_mst[i] && min_cost[i] < min_distance {
                    // u 为距离MST最近的未访问节点
                    min_distance = min_cost[i];
                    u = i;
                }
            }

            // 将节点u加入MST
            in_mst[u] = true;
            total_cost += min_distance;

            // 更新其他节点到MST的距离
            // 每次加入u时，更新下其他节点到MST的最小距离，得到的就是未加入MST的节点到MST所有节点中的最短距离
            for v in 0..n {
                if !in_mst[v] {
                    let distance =
                        (points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs();
                    if distance < min_cost[v] {
                        min_cost[v] = distance;
                    }
                }
            }
        }

        total_cost
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
