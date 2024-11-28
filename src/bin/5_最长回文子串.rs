/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 *
 * https://leetcode.cn/problems/longest-palindromic-substring/description/
 *
 * category: algorithms
 * tags: string,dynamic-programming
 * Medium (38.82%)
 * Likes:    7395
 * Dislikes: 0
 * Total Accepted:    1.8M
 * Total Submissions: 4.7M
 * Testcase Example:  '"babad"'
 *
 * 给你一个字符串 s，找到 s 中最长的 回文 子串。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "babad"
 * 输出："bab"
 * 解释："aba" 同样是符合题意的答案。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "cbbd"
 * 输出："bb"
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 1000
 * s 仅由数字和英文字母组成
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let bytes = s.as_bytes();
        // dp[i][j] 表示 s[i..=j] 是否是回文串
        // dp[i][j] = dp[i+1][j-1] && s[i] == s[j]
        let mut dp = vec![vec![false; n]; n];

        let mut start = 0;
        let mut max_len = 1;
        for i in 0..n {
            // 单个字符是回文串
            dp[i][i] = true;
        }

        for len in 2..=n {
            // s 表示子串的起始位置，e 表示子串的结束位置
            for s in 0..n {
                let e = s + len - 1;
                if e >= n {
                    break;
                }
                if bytes[s] != bytes[e] {
                    dp[s][e] = false;
                } else {
                    if e - s < 3 {
                        dp[s][e] = true;
                    } else {
                        dp[s][e] = dp[s + 1][e - 1];
                    }
                }
                if dp[s][e] && e - s + 1 > max_len {
                    start = s;
                    max_len = e - s + 1;
                }
            }
        }
        s[start..start + max_len].to_string()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
}
