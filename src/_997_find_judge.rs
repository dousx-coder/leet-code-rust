///
/// [997. 小镇的法官](https://leetcode.cn/problems/find-the-town-judge/?envType=problem-list-v2&envId=graph)
///
struct Solution;
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        //  入度数组
        let mut in_degree = vec![0; n as usize + 1];
        //  出度数组
        let mut out_degree = vec![0; n as usize + 1];
        for edge in trust.iter() {
            out_degree[edge[0] as usize] += 1;
            in_degree[edge[1] as usize] += 1;
        }

        for i in 1..=n as usize {
            if out_degree[i] == 0 && in_degree[i] == n - 1 {
                // 入度为n-1
                // 出度为0
                return i as i32;
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
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
    }
}
