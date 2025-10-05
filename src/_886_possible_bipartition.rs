///
/// [886. 可能的二分法](https://leetcode.cn/problems/possible-bipartition/?envType=problem-list-v2&envId=graph)
///
struct Solution;
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let len = n + 1;
        // 0表示不讨厌,1表示讨厌
        let mut graph = vec![vec![0; len]; len];
        // 0表示未访问,1表示分组A,-1表示分组B
        let mut colors = vec![0; len];

        for edge in &dislikes {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u][v] = 1;
            graph[v][u] = 1;
        }
        for u in 1..=n {
            if colors[u] == 0 && !Solution::dfs(&graph, &mut colors, n, u, 1) {
                return false;
            }
        }

        let sum = colors
            .iter()
            .map(|color| color.abs() as usize)
            .sum::<usize>();
        sum == n
    }
    fn dfs(graph: &Vec<Vec<i32>>, colors: &mut Vec<i32>, n: usize, u: usize, color: i32) -> bool {
        colors[u] = color;
        for j in 1..=n {
            if graph[u][j] == 1 {
                if colors[j] == color {
                    // 如果j和u在同一组，则说明发生了冲突
                    return false;
                }
                if colors[j] == 0 && !Solution::dfs(graph, colors, n, j, -color) {
                    // 尝试将j染成反色，如果不能染成反色，则不能继续
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]),
            true
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            false
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::possible_bipartition(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
            ),
            false
        );
    }
}
