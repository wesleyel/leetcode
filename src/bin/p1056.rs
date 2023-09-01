/**
 * [1056] Confusing Number
 *
 * A confusing number is a number that when rotated 180 degrees becomes a different number with each digit valid.
 *
 * We can rotate digits of a number by 180 degrees to form new digits.
 *
 *
 * 	When 0, 1, 6, 8, and 9 are rotated 180 degrees, they become 0, 1, 9, 8, and 6 respectively.
 * 	When 2, 3, 4, 5, and 7 are rotated 180 degrees, they become invalid.
 *
 *
 * Note that after rotating a number, we can ignore leading zeros.
 *
 *
 * 	For example, after rotating 8000, we have 0008 which is considered as just 8.
 *
 *
 * Given an integer n, return true if it is a confusing number, or false otherwise.
 *
 *
 * Example 1:
 *
 * Input: n = 6
 * Output: true
 * Explanation: We get 9 after rotating 6, 9 is a valid number, and 9 != 6.
 *
 *
 * Example 2:
 *
 * Input: n = 89
 * Output: true
 * Explanation: We get 68 after rotating 89, 68 is a valid number and 68 != 89.
 *
 *
 * Example 3:
 *
 * Input: n = 11
 * Output: false
 * Explanation: We get 11 after rotating 11, 11 is a valid number but the value remains the same, thus 11 is not a confusing number
 *
 *
 *
 * Constraints:
 *
 *
 * 	0 <= n <= 10⁹
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/confusing-number/
// discussion: https://leetcode.cn/problems/confusing-number/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn confusing_number(n: i32) -> bool {
        let (mut num, mut ans) = (n, 0);
        while num > 0 {
            match num % 10 {
                6 => ans = ans * 10 + 9,
                9 => ans = ans * 10 + 6,
                x @ (0 | 1 | 8) => ans = ans * 10 + x,
                _ => return false,
            }
            num /= 10;
        }
        ans != n
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confusing_number() {
        assert_eq!(Solution::confusing_number(6), true);
        assert_eq!(Solution::confusing_number(89), true);
        assert_eq!(Solution::confusing_number(11), false);
    }
}
