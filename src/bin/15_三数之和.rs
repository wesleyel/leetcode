/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 *
 * https://leetcode.cn/problems/3sum/description/
 *
 * category: algorithms
 * tags: array,two-pointers
 * Medium (38.54%)
 * Likes:    7107
 * Dislikes: 0
 * Total Accepted:    2M
 * Total Submissions: 5.2M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j !=
 * k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
 *
 * 注意：答案中不可以包含重复的三元组。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [-1,0,1,2,-1,-4]
 * 输出：[[-1,-1,2],[-1,0,1]]
 * 解释：
 * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
 * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
 * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
 * 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
 * 注意，输出的顺序和三元组的顺序并不重要。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0,1,1]
 * 输出：[]
 * 解释：唯一可能的三元组和不为 0 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [0,0,0]
 * 输出：[[0,0,0]]
 * 解释：唯一可能的三元组和为 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= nums.length <= 3000
 * -10^5 <= nums[i] <= 10^5
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = vec![];
        let n = nums.len();
        if n < 3 {
            return res;
        }
        nums.sort();
        for i in 0..n {
            if nums[i] > 0 {
                // 第一个数大于 0，后面的数都比它大，三数之和不可能为 0
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                // 去重
                continue;
            }
            // 双指针
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(Solution::three_sum(nums), res);

        let nums = vec![0, 1, 1];
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(nums), res);

        let nums = vec![0, 0, 0];
        let res = vec![vec![0, 0, 0]];
        assert_eq!(Solution::three_sum(nums), res);
    }
}
