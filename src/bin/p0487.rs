/**
 * [487] Max Consecutive Ones II
 *
 * Given a binary array nums, return the maximum number of consecutive 1's in the array if you can flip at most one 0.
 *
 *
 * Example 1:
 *
 * Input: nums = [1,0,1,1,0]
 * Output: 4
 * Explanation:
 * - If we flip the first zero, nums becomes [1,1,1,1,0] and we have 4 consecutive ones.
 * - If we flip the second zero, nums becomes [1,0,1,1,1] and we have 3 consecutive ones.
 * The max number of consecutive ones is 4.
 *
 *
 * Example 2:
 *
 * Input: nums = [1,0,1,1,0,1]
 * Output: 4
 * Explanation:
 * - If we flip the first zero, nums becomes [1,1,1,1,0,1] and we have 4 consecutive ones.
 * - If we flip the second zero, nums becomes [1,0,1,1,1,1] and we have 4 consecutive ones.
 * The max number of consecutive ones is 4.
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 10⁵
 * 	nums[i] is either 0 or 1.
 *
 *
 *
 * Follow up: What if the input numbers come in one by one as an infinite stream? In other words, you can't store all numbers coming from the stream as it's too large to hold in memory. Could you solve it efficiently?
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/max-consecutive-ones-ii/
// discussion: https://leetcode.cn/problems/max-consecutive-ones-ii/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut max, mut prev, mut cnt) = (0, 0, 0);
        for i in nums.iter() {
            cnt += 1;
            if i == &0 {
                prev = cnt; /* 连续1的个数 + 1个0 */
                cnt = 0;
            }
            max = max.max(prev + cnt); /* 上次连续+本次连续 */
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_487() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0]), 4);
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            4
        );
    }
}
