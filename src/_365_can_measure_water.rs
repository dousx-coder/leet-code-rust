use std::collections::{HashSet, VecDeque};
///
/// [365. 水壶问题](https://leetcode.cn/problems/water-and-jug-problem/)
///
struct Solution;
impl Solution {
    /// 向队列中添加元素
    fn push_queue(
        tuple: (i32, i32),
        target: i32,
        queue: &mut VecDeque<(i32, i32)>,
        added: &mut HashSet<(i32, i32)>,
    ) {
        if added.insert(tuple) {
            queue.push_back(tuple);
        }
    }

    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        if target == 0 {
            return true;
        }
        if x + y < target {
            return false;
        }
        if x == target || y == target || x + y == target {
            return true;
        }
        // bfs
        let mut added = HashSet::new();
        let mut queue = VecDeque::new();
        let start = (0, 0);
        queue.push_back(start);
        while !queue.is_empty() {
            let pop = queue.pop_front().unwrap();
            let curr_x = pop.0;
            let curr_y = pop.1;
            if curr_x == target || curr_y == target || curr_x + curr_y == target {
                return true;
            }
            if curr_x == 0 {
                // 填满第1个桶
                Self::push_queue((x, curr_y), target, &mut queue, &mut added);
            }
            if curr_y == 0 {
                // 填满第2个桶
                Self::push_queue((curr_x, y), target, &mut queue, &mut added);
            }
            if curr_y < y {
                // 第1个桶倒空
                Self::push_queue((0, curr_y), target, &mut queue, &mut added);
            }
            if curr_x < x {
                // 第2个桶倒空
                Self::push_queue((curr_x, 0), target, &mut queue, &mut added);
            }

            // y - curr_y是第二个桶还可以再加的水的升数，但是最多只能加curr_x升水。
            let min_move = curr_x.min(y - curr_y);
            // 把第1个桶里的水倒入第2个桶里
            Self::push_queue(
                (curr_x - min_move, curr_y + min_move),
                target,
                &mut queue,
                &mut added,
            );
            // 反过来同理，x - curr_x是第一个桶还可以再加的升数，但是最多只能加curr_y升水。
            let min_move = curr_y.min(x - curr_x);
            // 把第2个桶里的水倒入第1个桶里
            Self::push_queue(
                (curr_x + min_move, curr_y - min_move),
                target,
                &mut queue,
                &mut added,
            );
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // 按照以下步骤操作，以达到总共 4 升水：
        // 1. 装满 5 升的水壶(0, 5)。
        // 2. 把 5 升的水壶倒进 3 升的水壶，留下 2 升(3, 2)。
        // 3. 倒空 3 升的水壶(0, 2)。
        // 4. 把 2 升水从 5 升的水壶转移到 3 升的水壶(2, 0)。
        // 5. 再次加满 5 升的水壶(2, 5)。
        // 6. 从 5 升的水壶向 3 升的水壶倒水直到 3 升的水壶倒满。5 升的水壶里留下了 4 升水(3, 4)。
        // 7. 倒空 3 升的水壶。现在，5 升的水壶里正好有 4 升水(0, 4)。
        assert_eq!(Solution::can_measure_water(3, 5, 4), true);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::can_measure_water(2, 6, 5), false);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::can_measure_water(1, 1, 1), true);
    }
}
