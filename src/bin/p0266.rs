/**
 * [266] Palindrome Permutation
 *
 * Given a string s, return true if a permutation of the string could form a palindrome and false otherwise.
 *
 *
 * Example 1:
 *
 * Input: s = "code"
 * Output: false
 *
 *
 * Example 2:
 *
 * Input: s = "aab"
 * Output: true
 *
 *
 * Example 3:
 *
 * Input: s = "carerac"
 * Output: true
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= s.length <= 5000
 * 	s consists of only lowercase English letters.
 *
 *
 */
pub struct Solution {}
// problem: https://leetcode.cn/problems/palindrome-permutation/
// discussion: https://leetcode.cn/problems/palindrome-permutation/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        /* 统计字符出现次数，判断奇数个数是否唯一 */
        let mut map: HashMap<char, i32> = HashMap::new();
        for i in s.chars().into_iter() {
            let val = map.get(&i).unwrap_or(&0) + 1;
            map.insert(i, val);
        }
        let cnt = map
            .iter()
            .fold(0, |acc, (_, val)| if *val % 2 == 1 { acc + 1 } else { acc });
        cnt == 1 || cnt == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_266() {
        assert_eq!(Solution::can_permute_palindrome("aa".to_string()), true);
        assert_eq!(Solution::can_permute_palindrome("code".to_string()), false);
        assert_eq!(Solution::can_permute_palindrome("aab".to_string()), true);
        assert_eq!(
            Solution::can_permute_palindrome("carerac".to_string()),
            true
        );
    }
}
