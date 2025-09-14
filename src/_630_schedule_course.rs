use std::collections::BinaryHeap;

///
/// [630. 课程表Ⅲ](https://leetcode.cn/problems/course-schedule-iii/)
///
struct Solution;
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by(|a, b| a[1].cmp(&b[1]));
        //  大顶堆
        let mut heap = BinaryHeap::new();
        // 累计所需时长
        let mut total_duration = 0;
        for (index, course) in courses.iter().enumerate() {
            let duration = course[0];
            let last_day = course[1];
            if duration > last_day {
                continue;
            }
            if total_duration + duration <= last_day {
                total_duration += duration;
                heap.push(duration);
            } else {
                if !heap.is_empty() && *heap.peek().unwrap() > duration {
                    // 判断已经加入的最长课程是否可以被替换
                    total_duration += duration - *heap.peek().unwrap();
                    heap.pop();
                    heap.push(duration);
                }
            }
        }

        heap.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::schedule_course(vec![
                vec![100, 200],
                vec![200, 1300],
                vec![1000, 1250],
                vec![2000, 3200]
            ]),
            3
        );
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::schedule_course(vec![vec![1, 2]]), 1);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
    }
}
