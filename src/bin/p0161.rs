/**
 * [161] One Edit Distance
 *
 * Given two strings s and t, return true if they are both one edit distance apart, otherwise return false.
 *
 * A string s is said to be one distance apart from a string t if you can:
 *
 *
 * 	Insert exactly one character into s to get t.
 * 	Delete exactly one character from s to get t.
 * 	Replace exactly one character of s with a different character to get t.
 *
 *
 *
 * Example 1:
 *
 * Input: s = "ab", t = "acb"
 * Output: true
 * Explanation: We can insert 'c' into s to get t.
 *
 *
 * Example 2:
 *
 * Input: s = "", t = ""
 * Output: false
 * Explanation: We cannot get t from s by only one step.
 *
 *
 *
 * Constraints:
 *
 *
 * 	0 <= s.length, t.length <= 10⁴
 * 	s and t consist of lowercase letters, uppercase letters, and digits.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/one-edit-distance/
// discussion: https://leetcode.cn/problems/one-edit-distance/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let (len_s, len_t) = (s.len(), t.len());
        if len_s < len_t {
            return Solution::is_one_edit_distance(t, s);
        } else if len_s - len_t > 1 {
            return false;
        }

        let mut found = false;
        for (i, (x, y)) in s.bytes().zip(t.bytes()).enumerate() {
            if x != y {
                found = true;
                if len_s == len_t {
                    return s.as_bytes()[i + 1..] == t.as_bytes()[i + 1..];
                } else {
                    return s.as_bytes()[i + 1..] == t.as_bytes()[i..];
                }
            }
        }
        found || len_s - len_t == 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_one_edit_distance() {
        let s = "ab".to_string();
        let t = "acb".to_string();
        assert_eq!(Solution::is_one_edit_distance(s, t), true)
    }

    #[test]
    fn test_is_one_edit_distance2() {
        let s = "cab".to_string();
        let t = "ad".to_string();
        assert_eq!(Solution::is_one_edit_distance(s, t), false)
    }
}
