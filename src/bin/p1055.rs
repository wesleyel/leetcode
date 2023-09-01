/**
 * [1055] Shortest Way to Form String
 *
 * A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
 *
 * Given two strings source and target, return the minimum number of subsequences of source such that their concatenation equals target. If the task is impossible, return -1.
 *
 *
 * Example 1:
 *
 * Input: source = "abc", target = "abcbc"
 * Output: 2
 * Explanation: The target "abcbc" can be formed by "abc" and "bc", which are subsequences of source "abc".
 *
 *
 * Example 2:
 *
 * Input: source = "abc", target = "acdbc"
 * Output: -1
 * Explanation: The target string cannot be constructed from the subsequences of source string due to the character "d" in target string.
 *
 *
 * Example 3:
 *
 * Input: source = "xyz", target = "xzyxz"
 * Output: 3
 * Explanation: The target string can be constructed as follows "xz" + "y" + "xz".
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= source.length, target.length <= 1000
 * 	source and target consist of lowercase English letters.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/shortest-way-to-form-string/
// discuss: https://leetcode.cn/problems/shortest-way-to-form-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let (mut cnt, lt) = (0, target.len());
        let mut it = 0;
        while it < lt {
            let ori = it;
            for s in source.chars() {
                if it < lt && target.as_bytes()[it] == s as u8 {
                    it += 1;
                }
            }
            if ori == it {
                return -1;
            }
            cnt += 1;
        }
        cnt
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_way() {
        let s = "abc".to_string();
        let t = "abcbc".to_string();
        assert_eq!(Solution::shortest_way(s, t), 2);
    }
}
