/*
 * @lc app=leetcode.cn id=3185 lang=rust
 *
 * [3185] 构成整天的下标对数目 II
 *
 * https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-ii/description/
 *
 * algorithms
 * Medium (53.67%)
 * Likes:    26
 * Dislikes: 0
 * Total Accepted:    22.7K
 * Total Submissions: 39.3K
 * Testcase Example:  '[12,12,30,24,24]'
 *
 * 给你一个整数数组 hours，表示以 小时 为单位的时间，返回一个整数，表示满足 i < j 且 hours[i] + hours[j] 构成 整天
 * 的下标对 i, j 的数目。
 *
 * 整天 定义为时间持续时间是 24 小时的 整数倍 。
 *
 * 例如，1 天是 24 小时，2 天是 48 小时，3 天是 72 小时，以此类推。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入： hours = [12,12,30,24,24]
 *
 * 输出： 2
 *
 * 解释：
 *
 * 构成整天的下标对分别是 (0, 1) 和 (3, 4)。
 *
 *
 * 示例 2：
 *
 *
 * 输入： hours = [72,48,24,3]
 *
 * 输出： 3
 *
 * 解释：
 *
 * 构成整天的下标对分别是 (0, 1)、(0, 2) 和 (1, 2)。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= hours.length <= 5 * 10^5
 * 1 <= hours[i] <= 10^9
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut res = 0i64;
        let mut bitmap = [0u32; 24];
        for &hour in &hours {
            let h = hour as usize % 24;
            res += bitmap[(24 - h) % 24] as i64;
            bitmap[h] += 1;
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
        let hours = vec![12, 12, 30, 24, 24];
        assert_eq!(Solution::count_complete_day_pairs(hours), 2);

        let hours = vec![72, 48, 24, 3];
        assert_eq!(Solution::count_complete_day_pairs(hours), 3);
    }
}
