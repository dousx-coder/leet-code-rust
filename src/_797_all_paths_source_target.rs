///
/// [797. 所有可能的路径](https://leetcode.cn/problems/all-paths-from-source-to-target/)
///
struct Solution;
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::dfs_solution(graph)
    }
    fn dfs_solution(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let n = graph.len();
        let mut curr = vec![0];
        Self::dfs(&graph, &mut ans, &mut curr, 0);
        ans
    }
    fn dfs(graph: &Vec<Vec<i32>>, path: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>, index: usize) {
        if index == graph.len() - 1 {
            path.push(curr.clone());
            return;
        }
        for ele in &graph[index] {
            curr.push(*ele);
            Self::dfs(graph, path, curr, *ele as usize);
            curr.pop();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
            vec![vec![0, 1, 3], vec![0, 2, 3]]
        );
    }
}
