/// 853 车队
///
/// https://leetcode.cn/problems/car-fleet/?envType=problem-list-v2&envId=monotonic-stack
struct Solution;
impl Solution {
    ///  `position[i]` 是第 `i` 辆车的位置， `speed[i]` 是第 `i` 辆车的速度(单位是英里/小时)
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let len = position.len();
        if len <= 1 {
            return len as i32;
        }
        // 计算出每个车辆到达终点还需多少时间(精确到浮点)
        let mut elapsed_time = vec![];
        for i in 0..len {
            let surplus = (target - position[i]) as f32;
            elapsed_time.push((position[i], surplus / speed[i] as f32));
        }
        // 按照离终点距离由近到远进行排序，离得近，所需时间长，和离的远，所需时间短的车辆是一个车队的
        elapsed_time.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let mut car_fleet_num = 1;
        let mut elapsed = (&elapsed_time[0]).1;
        for i in 0..elapsed_time.len() {
            let x = &elapsed_time[i];
            if x.1 > elapsed {
                car_fleet_num += 1;
                elapsed = x.1;
            }
        }
        car_fleet_num
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
    }
}
