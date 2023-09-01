/**
 * [1427] Perform String Shifts
 *
 * You are given a string s containing lowercase English letters, and a matrix shift, where shift[i] = [directioni, amounti]:
 *
 *
 * 	directioni can be 0 (for left shift) or 1 (for right shift).
 * 	amounti is the amount by which string s is to be shifted.
 * 	A left shift by 1 means remove the first character of s and append it to the end.
 * 	Similarly, a right shift by 1 means remove the last character of s and add it to the beginning.
 *
 *
 * Return the final string after all operations.
 *
 *
 * Example 1:
 *
 * Input: s = "abc", shift = [[0,1],[1,2]]
 * Output: "cab"
 * Explanation:
 * [0,1] means shift to left by 1. "abc" -> "bca"
 * [1,2] means shift to right by 2. "bca" -> "cab"
 *
 * Example 2:
 *
 * Input: s = "abcdefg", shift = [[1,1],[1,1],[0,2],[1,3]]
 * Output: "efgabcd"
 * Explanation:
 * [1,1] means shift to right by 1. "abcdefg" -> "gabcdef"
 * [1,1] means shift to right by 1. "gabcdef" -> "fgabcde"
 * [0,2] means shift to left by 2. "fgabcde" -> "abcdefg"
 * [1,3] means shift to right by 3. "abcdefg" -> "efgabcd"
 *
 *
 * Constraints:
 *
 *
 * 	1 <= s.length <= 100
 * 	s only contains lower case English letters.
 * 	1 <= shift.length <= 100
 * 	shift[i].length == 2
 * 	directioni is either 0 or 1.
 * 	0 <= amounti <= 100
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/perform-string-shifts/
// discuss: https://leetcode.cn/problems/perform-string-shifts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let mut index = shift
            .iter()
            .fold(0, |acc, x| acc + i32::pow(-1, x[0] as u32) * x[1]);
        let len = s.len() as i32;
        index = (index % len + len) % len;
        format!("{}{}", &s[index as usize..], &s[..index as usize])
    }
}

macro_rules!vec2d {
    [ $( [ $( $d:expr ),* ] ),* ] => {
        vec![
            $(
                vec![$($d),*],
            )*
        ]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_shift() {
        let src = "abc".to_string();
        let exp: String = "cab".to_string();
        assert_eq!(
            Solution::string_shift(src, vec![vec![0, 1], vec![1, 2]]),
            exp
        );
    }

    #[test]
    fn test_string_shift2() {
        let src = "yisxjwry".to_string();
        let exp: String = "yisxjwry".to_string();
        assert_eq!(
            Solution::string_shift(
                src,
                vec![
                    vec![1, 8],
                    vec![1, 4],
                    vec![1, 3],
                    vec![1, 6],
                    vec![0, 6],
                    vec![1, 4],
                    vec![0, 2],
                    vec![0, 1]
                ]
            ),
            exp
        );
    }

    #[test]
    fn test_string_shift3() {
        let src = "xqgwkiqpif".to_string();
        let exp: String = "qpifxqgwki".to_string();
        assert_eq!(
            Solution::string_shift(
                src,
                vec2d![
                    [1, 4],
                    [0, 7],
                    [0, 8],
                    [0, 7],
                    [0, 6],
                    [1, 3],
                    [0, 1],
                    [1, 7],
                    [0, 5],
                    [0, 6]
                ]
            ),
            exp
        );
    }
}
