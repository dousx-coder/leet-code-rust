///
/// [200. 岛屿数量](https://leetcode.cn/problems/number-of-islands/description/?envType=problem-list-v2&envId=depth-first-search)
///
struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut fire: Vec<Vec<bool>> = vec![];
        //  m行n列
        let m = grid.len();
        // 二维数组长度一样，这里取第0个即可
        let n = grid[0].len();
        for i in 0..grid.len() {
            fire.push(vec![false; grid[i].len()]);
        }
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if fire[i][j] {
                    continue;
                }
                if Solution::is_land(&grid[i][j]) {
                    //开始放火
                    Solution::fire(i, j, m, n, &mut fire, &grid);
                    count += 1;
                }
            }
        }
        count
    }
    fn fire(
        i: usize,
        j: usize,
        m: usize,
        n: usize,
        fire: &mut Vec<Vec<bool>>,
        grid: &Vec<Vec<char>>,
    ) {
        if i >= m || j >= n {
            return;
        }
        if fire[i][j] {
            return;
        }
        if Solution::is_land(&grid[i][j]) {
            fire[i][j] = true;
            Solution::fire(i + 1, j, m, n, fire, grid);
            Solution::fire(i, j + 1, m, n, fire, grid);
            if i >= 1 {
                Solution::fire(i - 1, j, m, n, fire, grid);
            }
            if j >= 1 {
                Solution::fire(i, j - 1, m, n, fire, grid);
            }
        } else {
            return;
        }
    }
    fn is_land(ch: &char) -> bool {
        *ch == '1'
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
