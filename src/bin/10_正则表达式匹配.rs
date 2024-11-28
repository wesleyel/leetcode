/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 *
 * https://leetcode.cn/problems/regular-expression-matching/description/
 *
 * category: algorithms
 * tags: string,dynamic-programming,backtracking
 * Hard (30.74%)
 * Likes:    3985
 * Dislikes: 0
 * Total Accepted:    438.2K
 * Total Submissions: 1.4M
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
 *
 *
 * '.' 匹配任意单个字符
 * '*' 匹配零个或多个前面的那一个元素
 *
 *
 * 所谓匹配，是要涵盖 整个 字符串 s 的，而不是部分字符串。
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "aa", p = "a"
 * 输出：false
 * 解释："a" 无法匹配 "aa" 整个字符串。
 *
 *
 * 示例 2:
 *
 *
 * 输入：s = "aa", p = "a*"
 * 输出：true
 * 解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "ab", p = ".*"
 * 输出：true
 * 解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 20
 * 1 <= p.length <= 20
 * s 只包含从 a-z 的小写字母。
 * p 只包含从 a-z 的小写字母，以及字符 . 和 *。
 * 保证每次出现字符 * 时，前面都匹配到有效的字符
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn is_match_dp(s: String, p: String) -> bool {
        let str = s.as_bytes();
        let pat = p.as_bytes();
        let len_str = str.len();
        let len_pat = pat.len();
        // dp[i][j] 表示 s 的前 i 个字符和 p 的前 j 个字符是否匹配
        // dp[i][j] = dp[i-1][j-1] && (s[i] == p[j] || p[j] == '.')
        let mut dp = vec![vec![false; len_pat + 1]; len_str + 1];
        dp[0][0] = true; // 空字符串永远匹配成功
        for i in 0..=len_str {
            for j in 1..=len_pat {
                if pat[j - 1] == b'*' {
                    // pat[j - 1] = * 时
                    // 1. 表示0个前置字符，按照dp[i][j - 2]处理
                    // 2. 表示1个或多个前置字符，按照dp[i][j - 1]处理
                    dp[i][j] = dp[i][j - 2]
                        || (i > 0
                            && (str[i - 1] == pat[j - 2] || pat[j - 2] == b'.')
                            && dp[i - 1][j]);
                } else {
                    // pat[j - 1] != * 时
                    // 按照dp[i][j]处理
                    dp[i][j] = i > 0
                        && dp[i - 1][j - 1]
                        && (str[i - 1] == pat[j - 1] || pat[j - 1] == b'.');
                }
            }
        }
        dp[len_str][len_pat]
    }

    pub fn is_match(s: String, p: String) -> bool {
        // https://leetcode.com/problems/regular-expression-matching/solutions/292797/rust-with-pattern/
        let str = s.as_bytes();
        let pat = p.as_bytes();

        enum Pattern {
            Empty,      // pat 为空
            Single(u8), // pat 为单个字符
            Repeat(u8), // pat 为单个字符加 *
        }

        /// 从左到右，解析第一个pat的类型
        fn parse_pattern(pat: &[u8]) -> (Pattern, &[u8]) {
            match pat {
                [] => (Pattern::Empty, &[]),
                [c, b'*', rest @ ..] => (Pattern::Repeat(*c), rest),
                [c, rest @ ..] => (Pattern::Single(*c), rest),
            }
        }

        /// 消耗str的第一个字符，根据c的模式进行匹配
        fn consume_str(str: &[u8], c: u8, pat: &[u8]) -> bool {
            match str {
                [] => false,
                [s, rest @ ..] => (c == b'.' || c == *s) && consume_pat(rest, pat),
            }
        }

        /// 消耗pat的一个模式，根据类型进行匹配
        fn consume_pat(str: &[u8], pat: &[u8]) -> bool {
            match parse_pattern(pat) {
                // pat 为空
                (Pattern::Empty, _) => str.is_empty(),
                // pat 为单个字符c
                (Pattern::Single(c), rest) => consume_str(str, c, rest),
                // pat 为单个字符c加 *, 如果不能匹配，尝试匹配下一个模式
                (Pattern::Repeat(c), rest) => consume_str(str, c, pat) || consume_pat(str, rest),
            }
        }

        consume_pat(str, pat)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
    }
}
