/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 *
 * https://leetcode.cn/problems/reverse-integer/description/
 *
 * algorithms
 * Medium (35.56%)
 * Likes:    4023
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 3.8M
 * Testcase Example:  '123'
 *
 * 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
 *
 * 如果反转后整数超过 32 位的有符号整数的范围 [−2^31,  2^31 − 1] ，就返回 0。
 * 假设环境不允许存储 64 位整数（有符号或无符号）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：x = 123
 * 输出：321
 *
 *
 * 示例 2：
 *
 *
 * 输入：x = -123
 * 输出：-321
 *
 *
 * 示例 3：
 *
 *
 * 输入：x = 120
 * 输出：21
 *
 *
 * 示例 4：
 *
 *
 * 输入：x = 0
 * 输出：0
 *
 *
 *
 *
 * 提示：
 *
 *
 * -2^31
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // 反转字符串然后转换为数字
        // 使用checked_mul和checked_add来检查溢出
        let s = x.to_string();
        s.chars()
            .rev()
            .fold(Some(0), |acc, c| {
                acc.and_then(|a: i32| {
                    if c == '-' {
                        Some(-a)
                    } else {
                        a.checked_mul(10)?.checked_add(c.to_digit(10)? as i32)
                    }
                })
            })
            .unwrap_or(0)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(i32::MAX), 0);
    }
}
