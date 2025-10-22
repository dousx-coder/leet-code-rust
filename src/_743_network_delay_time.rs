use std::cmp::Reverse;
use std::collections::BinaryHeap;
///
/// [743. 网络延迟时间](https://leetcode.cn/problems/network-delay-time/?envType=problem-list-v2&envId=graph)
///
struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        // 构建邻接表
        let mut graph = vec![vec![]; n + 1];
        for time in times {
            let u = time[0] as usize;
            let v = time[1] as usize;
            let w = time[2];
            graph[u].push((v, w));
        }

        // 距离数组
        let mut dist = vec![i32::MAX; n + 1];
        dist[k] = 0;

        // 优先队列，存储(距离, 节点)
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, k)));

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue;
            }

            // 遍历u的所有邻居
            for &(v, w) in &graph[u] {
                let new_dist = d + w;
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    pq.push(Reverse((new_dist, v)));
                }
            }
        }

        // 检查是否所有节点都可达
        let mut max_time = 0;
        for i in 1..=n {
            if dist[i] == i32::MAX {
                return -1;
            }
            max_time = max_time.max(dist[i]);
        }

        max_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        let n = 4;
        let k = 2;
        assert_eq!(Solution::network_delay_time(times, n, k), 2);
    }

    #[test]
    fn t2() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 1;
        assert_eq!(Solution::network_delay_time(times, n, k), 1);
    }

    #[test]
    fn t3() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 2;
        assert_eq!(Solution::network_delay_time(times, n, k), -1);
    }

    #[test]
    fn t4() {
        let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]];
        let n = 3;
        let k = 1;
        assert_eq!(Solution::network_delay_time(times, n, k), 3);
    }
}
