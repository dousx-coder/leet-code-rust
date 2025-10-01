///
/// [1971. 寻找图中是否存在路径](https://leetcode.cn/problems/find-if-path-exists-in-graph/)
///
struct Solution;
impl Solution {
    /// 并查集解题
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut father = vec![0; n];
        for i in 0..n {
            father[i] = i;
        }
        edges.iter().enumerate().for_each(|(_, edge)| {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            Self::join(&mut father, u, v);
        });
        Self::find(&mut father, source as usize) == Self::find(&mut father, destination as usize)
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
            Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
            true
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::valid_path(
                6,
                vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
                0,
                5
            ),
            false
        );
    }
}
