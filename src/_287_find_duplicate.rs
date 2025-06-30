/// 287 寻找重复数
///
/// https://leetcode.cn/problems/find-the-duplicate-number/description/?envType=problem-list-v2&envId=binary-search
struct Solution;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // 弗洛伊德循环检测算法，也被称为龟兔赛跑算法
        // 数组的值作为快慢指针
        let mut tortoise = nums[0];
        let mut hare = nums[0];
        loop {
            tortoise = nums[tortoise as usize];
            hare = nums[nums[hare as usize] as usize];
            if tortoise == hare {
                // 快慢指针相遇的地方指的是指针所处的节点位置（索引）相同
                break;
            }
        }
        let mut ptr1 = nums[0];
        let mut ptr2 = tortoise;
        while ptr1 != ptr2 {
            ptr1 = nums[ptr1 as usize];
            ptr2 = nums[ptr2 as usize];
        }
        ptr1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::find_duplicate(vec![3, 3, 3, 3, 3]), 3);
    }

    #[test]
    fn t4() {
        //  1-->2-->3-->4-->5-->6-->7
        //                          7-->4
        //                              4-->5-->6-->7 (4,5,6,7是环)
        // tortoise: 1, hare: 1
        // tortoise: 2, hare: 3
        // tortoise: 3, hare: 5
        // tortoise: 4, hare: 7
        // tortoise: 5, hare: 5相遇
        //
        // ptr1: 1, ptr2: 5
        // ptr1: 2, ptr2: 6
        // ptr1: 3, ptr2: 7
        // ptr1: 4, ptr2: 4
        assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 4]), 4);
    }
}
