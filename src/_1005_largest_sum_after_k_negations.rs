/// 1005 K 次取反后最大化的数组和
///
/// https://leetcode.cn/problems/maximize-sum-of-array-after-k-negations/description/
struct Solution;
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let len = nums.len();
        let mut sum = 0;
        let mut k = k;
        // 如果是负数 则k消耗一次，如果是正数，k如果剩余次数是偶数，则消耗两次，
        for i in 0..len {
            if k <= 0 {
                break;
            };

            let n = nums[i];
            if n == 0 {
                break;
            }

            if nums[i] < 0 {
                // 取反
                nums[i] = -n;
                k -= 1;
                if k > 0 {
                    if i + 1 < len {
                        // k未用完，且下一个数字是正数，且当前数字的绝对值小于下一个数字，
                        if nums[i + 1] > 0 && nums[i].abs() < nums[i + 1].abs() {
                            // 根据剩余的次数的奇偶判断
                            // 当前数字的绝对值小于下一个数字，则应该将当前的数字设置为负数，下一个数字不动

                            // 1.如果次数剩余的k是奇数，
                            // k=3,num[i]=2, num[i+1]=3
                            //     num[i]=-2,num[i+1]=3  i消耗奇数次，给num[i+1]留2次
                            //

                            // 2.如果剩余偶数次
                            // k=4,num[i]=2, num[i+1]=3
                            //    则num[i] 不用动，后面都会保持数字不取反

                            if k % 2 == 1 {
                                nums[i] = n;
                                k -= 1;
                            } else {
                                // 提前结束循环
                                k = 0;
                            }
                        }
                    } else {
                        //当前数字是最后一个数字,要判断k是否为0
                        if k != 0 {
                            while k != 0 {
                                nums[i] = -nums[i];
                                k -= 1;
                            }
                        }
                    }
                }
            } else {
                while k >= 2 {
                    k -= 2;
                }
                if k == 1 {
                    nums[i] = -n;
                    break;
                }
            }
        }
        nums.iter().sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
    }
    #[test]
    fn t3() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![1, 3, 2, 6, 7, 9], 3),
            26
        );
    }

    #[test]
    fn t5() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![-8, 3, -5, -3, -5, -2], 6),
            22
        );
    }

    #[test]
    fn t6() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![-4, -2, -3], 4),
            5
        );
    }
}
