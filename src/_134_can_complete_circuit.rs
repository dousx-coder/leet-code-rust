/// 134 加油站
///
/// https://leetcode.cn/problems/gas-station/
struct Solution;
impl Solution {
    /// gas 加油站
    ///
    /// cost 消耗
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        // 剩余油量总和
        let mut total_sum = 0;
        let mut min_sum = i32::MAX;
        for i in 0..len {
            total_sum += gas[i] - cost[i];
            if total_sum < min_sum {
                min_sum = total_sum;
            }
        }
        if total_sum < 0 {
            //gas的总和小于cost总和，那么无论从哪里出发，一定是跑不了一圈的
            return -1;
        }
        if min_sum >= 0 {
            // rest[i] = gas[i]-cost[i]为一天剩下的油，
            // i从0开始计算累加到最后一站，如果累加没有出现负数，说明从0出发，油就没有断过，那么0就是起点
            return 0;
        }

        // 累加的最小值是负数，汽车就要从非0节点出发
        // 从后向前，看哪个节点能把这个负数填平，能把这个负数填平的节点就是出发节点
        for i in (0..len).rev() {
            min_sum += gas[i] - cost[i];
            total_sum += gas[i] - cost[i];
            if min_sum >= 0 {
                return i as i32;
            }
        }
        1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
