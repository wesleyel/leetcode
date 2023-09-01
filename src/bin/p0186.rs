/**
 * [186] Reverse Words in a String II
 *
 * Given a character array s, reverse the order of the words.
 *
 * A word is defined as a sequence of non-space characters. The words in s will be separated by a single space.
 *
 * Your code must solve the problem in-place, i.e. without allocating extra space.
 *
 *
 * Example 1:
 * Input: s = ["t","h","e"," ","s","k","y"," ","i","s"," ","b","l","u","e"]
 * Output: ["b","l","u","e"," ","i","s"," ","s","k","y"," ","t","h","e"]
 * Example 2:
 * Input: s = ["a"]
 * Output: ["a"]
 *
 *
 * Constraints:
 *
 *
 * 	1 <= s.length <= 10⁵
 * 	s[i] is an English letter (uppercase or lowercase), digit, or space ' '.
 * 	There is at least one word in s.
 * 	s does not contain leading or trailing spaces.
 * 	All the words in s are guaranteed to be separated by a single space.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/reverse-words-in-a-string-ii/
// discuss: https://leetcode.cn/problems/reverse-words-in-a-string-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        let len = s.len();
        Self::reverse_m_n(0, len - 1, s);

        let mut ia = 0;
        for ib in 0..len {
            if s[ib].is_ascii_whitespace() || ib == len - 1 {
                if ib == len - 1 {
                    Self::reverse_m_n(ia, ib, s);
                } else {
                    Self::reverse_m_n(ia, ib - 1, s);
                }
                ia = ib + 1;
            }
        }
    }

    fn reverse_m_n(mut m: usize, mut n: usize, s: &mut Vec<char>) {
        while m < n {
            s.swap(m, n);
            m += 1;
            n -= 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let mut s: Vec<char> = vec![
            't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
        ];
        let exp = vec![
            'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
        ];
        Solution::reverse_words(&mut s);
        assert_eq!(s, exp)
    }
}
