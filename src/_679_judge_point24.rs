///
/// 679 24点
///
/// https://leetcode.cn/problems/24-game/description/?envType=problem-list-v2&envId=backtracking
struct Solution;
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let cards = cards.iter().map(|x| *x as f64).collect::<Vec<f64>>();
        Self::backtracking(&cards)
    }
    fn compute(x: f64, y: f64) -> Vec<f64> {
        vec![x + y, x - y, y - x, x * y, y / x, x / y]
    }
    fn backtracking(va: &Vec<f64>) -> bool {
        let len = va.len();
        if len == 0 {
            return false;
        }
        if len == 1 {
            // 判断最终结果是否符合要求
            return (va[0] - 24f64).abs() < 0.01;
        }
        for i in 0..len {
            for j in i + 1..len {
                // 从数组中选2个数(va[i], va[j])进行计算
                let mut vb = vec![0f64; len - 1];
                let mut index = 0;
                for k in 0..len {
                    if k != i && k != j {
                        // 将剩余的数放到vb中
                        vb[index] = va[k];
                        index += 1;
                    }
                }
                for fd in Self::compute(va[i], va[j]) {
                    let n = vb.len();
                    vb[n - 1] = fd;
                    if Self::backtracking(&vb) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::judge_point24(vec![3, 3, 8, 8]), true);
    }
}
