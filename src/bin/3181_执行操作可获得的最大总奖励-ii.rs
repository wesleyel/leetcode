/*
 * @lc app=leetcode.cn id=3181 lang=rust
 *
 * [3181] 执行操作可获得的最大总奖励 II
 *
 * https://leetcode.cn/problems/maximum-total-reward-using-operations-ii/description/
 *
 * category: algorithms
 * tags: dynamic-programming
 * Hard (34.29%)
 * Likes:    12
 * Dislikes: 0
 * Total Accepted:    4.3K
 * Total Submissions: 11.6K
 * Testcase Example:  '[1,1,3,3]'
 *
 * 给你一个整数数组 rewardValues，长度为 n，代表奖励的值。
 *
 * 最初，你的总奖励 x 为 0，所有下标都是 未标记 的。你可以执行以下操作 任意次 ：
 *
 *
 * 从区间 [0, n - 1] 中选择一个 未标记 的下标 i。
 * 如果 rewardValues[i] 大于 你当前的总奖励 x，则将 rewardValues[i] 加到 x 上（即 x = x +
 * rewardValues[i]），并 标记 下标 i。
 *
 *
 * 以整数形式返回执行最优操作能够获得的 最大 总奖励。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：rewardValues = [1,1,3,3]
 *
 * 输出：4
 *
 * 解释：
 *
 * 依次标记下标 0 和 2，总奖励为 4，这是可获得的最大值。
 *
 *
 * 示例 2：
 *
 *
 * 输入：rewardValues = [1,6,4,3,2]
 *
 * 输出：11
 *
 * 解释：
 *
 * 依次标记下标 0、2 和 1。总奖励为 11，这是可获得的最大值。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= rewardValues.length <= 5 * 10^4
 * 1 <= rewardValues[i] <= 5 * 10^4
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
        reward_values.sort_unstable();
        reward_values.dedup();
        let max = reward_values.iter().max().unwrap().to_owned();
        let mut dp = vec![false; 2 * max as usize + 1];
        let dp = dp.as_mut_slice();
        dp[0] = true;

        for v in reward_values.into_iter() {
            for x in (v..v << 1).rev() {
                unsafe {
                    *(dp.get_unchecked_mut(x as usize)) |= *dp.get_unchecked((x - v) as usize);
                }
            }
        }
        dp.iter().enumerate().rfind(|(_, &x)| x).unwrap().0 as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let reward_values = vec![1, 1, 3, 3];
        assert_eq!(Solution::max_total_reward(reward_values), 4);
        let reward_values = vec![1, 6, 4, 3, 2];
        assert_eq!(Solution::max_total_reward(reward_values), 11);
    }
}
