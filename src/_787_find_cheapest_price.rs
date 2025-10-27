use std::cmp::Reverse;
use std::collections::BinaryHeap;

///
/// [787. K站中转内最便宜的航班](https://leetcode.cn/problems/cheapest-flights-within-k-stops/)
///
struct Solution;
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let dst = dst as usize;
        let src = src as usize;
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for flight in &flights {
            let from = flight[0] as usize;
            let to = flight[1] as usize;
            let price = flight[2];
            graph[from].push((to, price));
        }

        // 使用 visited 数组记录到达每个城市的最小花费和中转次数
        let mut visited = vec![(i32::MAX, 0); n];
        let mut pq = BinaryHeap::new();

        // (花费, 中转次数, 城市)
        pq.push(Reverse((0, 0, src)));
        visited[src].0 = 0;

        while let Some(Reverse((cost, stops, city))) = pq.pop() {
            if city == dst {
                return cost;
            }

            if stops > k {
                continue;
            }

            // 如果当前路径比已记录的更差，则跳过
            if cost > visited[city].0 && stops >= visited[city].1 {
                continue;
            }

            for &(next_city, price) in &graph[city] {
                let new_cost = cost + price;
                let new_stops = stops + 1;

                // 如果新路径更优，或者中转次数更少但花费可接受，则更新
                if new_cost < visited[next_city].0 || new_stops < visited[next_city].1 {
                    visited[next_city] = (new_cost, new_stops);
                    pq.push(Reverse((new_cost, new_stops, next_city)));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::find_cheapest_price(
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200],
            ],
            0,
            3,
            1,
        );
        assert_eq!(ans, 700);
    }

    #[test]
    fn t2() {
        let ans = Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1,
        );
        assert_eq!(ans, 200);
    }

    #[test]
    fn t3() {
        let ans = Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0,
        );
        assert_eq!(ans, 500);
    }

    #[test]
    fn t4() {
        let ans = Solution::find_cheapest_price(
            4,
            vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]],
            0,
            3,
            1,
        );
        assert_eq!(ans, 6);
    }
}
