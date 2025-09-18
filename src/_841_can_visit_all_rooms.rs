use std::collections::VecDeque;
///
/// [841. 钥匙和房间](https://leetcode.cn/problems/keys-and-rooms/?envType=problem-list-v2&envId=graph)
///
struct Solution;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut enter_room = vec![false; n];
        enter_room[0] = true;
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(i) = queue.pop_front() {
            for &j in rooms[i].iter() {
                let next = j as usize;
                if !enter_room[next] {
                    // 如果已经进入过了，这里不再进入避免队列进入死循环
                    enter_room[next] = true;
                    queue.push_back(next);
                }
            }
        }
        enter_room.iter().all(|&x| x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]),
            true
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]),
            false
        );
    }
}
