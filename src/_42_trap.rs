/// 42 接雨水
///
/// https://leetcode.cn/problems/trapping-rain-water/description/
struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len == 0 {
            return 0;
        }
        let mut left_max = vec![0; len];
        let mut right_max = vec![0; len];
        for i in 1..len {
            left_max[i] = left_max[i - 1].max(height[i - 1]);
            right_max[len - i - 1] = right_max[len - i].max(height[len - i]);
        }
        let mut result = 0;
        for i in 0..len {
            result += 0.max(left_max[i].min(right_max[i]) - height[i]);
        }
        result
    }

    /// 单调栈解法
    fn trap_monotonic_stack(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len == 0 {
            return 0;
        }

        let mut stack = Vec::with_capacity(len);
        let mut result = 0;
        // 纵坐标单位是10
        // (40)-        
        // (30)-       -
        // (20)- - -   -
        // (10)- - - - -
        //     0 1 2 3 4
        for i in 0..len {
            let curr_height = height[i];
            while !stack.is_empty() && height[*stack.last().unwrap()] < curr_height {
                let top = stack.pop().unwrap();
                if stack.is_empty() {
                    break;
                }
                // 当i为4时,curr_height=30
                // 第1次进入循环(top=3,left=2,height=10),此时栈内栈内元素为[0,1,2]
                // 第2次进入循环(top=2,left=1,height=0),此时栈内栈内元素为[0,1]
                // 第3次进入循环(top=1,left=0,height=10),此时栈为空，结束当前循环，将下标4压入栈，下次弹出后发现只有1个元素结束外层循环
                let left = *stack.last().unwrap();
                let min_height = curr_height.min(height[left]);
                let distance = (i - (left + 1)) as i32;
                let height = min_height - height[top];
                result += distance * height;
            }

            stack.push(i);
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::trap_monotonic_stack(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::trap_monotonic_stack(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::trap_monotonic_stack(vec![40, 20, 20, 10, 30]), 40);
    }
}
