/**
 * [1100] Find K-Length Substrings With No Repeated Characters
 *
 * Given a string s and an integer k, return the number of substrings in s of length k with no repeated characters.
 *
 *
 * Example 1:
 *
 * Input: s = "havefunonleetcode", k = 5
 * Output: 6
 * Explanation: There are 6 substrings they are: 'havef','avefu','vefun','efuno','etcod','tcode'.
 *
 *
 * Example 2:
 *
 * Input: s = "home", k = 5
 * Output: 0
 * Explanation: Notice k can be larger than the length of s. In this case, it is not possible to find any substring.
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= s.length <= 10⁴
 * 	s consists of lowercase English letters.
 * 	1 <= k <= 10⁴
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/find-k-length-substrings-with-no-repeated-characters/
// discussion: https://leetcode.cn/problems/find-k-length-substrings-with-no-repeated-characters/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let (mut left, mut res, mut map) = (0, 0, vec![0; 26]);
        for right in 0..s.len() {
            map[(s.as_bytes()[right] - b'a') as usize] += 1;
            if right >= k as usize {
                map[(s.as_bytes()[left] - b'a') as usize] -= 1;
                left += 1;
            }
            if map.iter().all(|&x| x <= 1) && right - left + 1 == k as usize {
                res += 1;
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1100() {
        assert_eq!(
            Solution::num_k_len_substr_no_repeats("havefunonleetcode".to_string(), 5),
            6
        );
        assert_eq!(
            Solution::num_k_len_substr_no_repeats("home".to_string(), 5),
            0
        );
    }
}
