/**
 * [1165] Single-Row Keyboard
 *
 * There is a special keyboard with all keys in a single row.
 *
 * Given a string keyboard of length 26 indicating the layout of the keyboard (indexed from 0 to 25). Initially, your finger is at index 0. To type a character, you have to move your finger to the index of the desired character. The time taken to move your finger from index i to index j is |i - j|.
 *
 * You want to type a string word. Write a function to calculate how much time it takes to type it with one finger.
 *
 *
 * Example 1:
 *
 * Input: keyboard = "abcdefghijklmnopqrstuvwxyz", word = "cba"
 * Output: 4
 * Explanation: The index moves from 0 to 2 to write 'c' then to 1 to write 'b' then to 0 again to write 'a'.
 * Total time = 2 + 1 + 1 = 4.
 *
 *
 * Example 2:
 *
 * Input: keyboard = "pqrstuvwxyzabcdefghijklmno", word = "leetcode"
 * Output: 73
 *
 *
 *
 * Constraints:
 *
 *
 * 	keyboard.length == 26
 * 	keyboard contains each English lowercase letter exactly once in some order.
 * 	1 <= word.length <= 10⁴
 * 	word[i] is an English lowercase letter.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/single-row-keyboard/
// discussion: https://leetcode.cn/problems/single-row-keyboard/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let map = vec![0; 26]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                (keyboard
                    .as_bytes()
                    .iter()
                    .position(|x| (x - b'a') as usize == i))
                .unwrap() as usize
            })
            .collect::<Vec<usize>>();
        word.as_bytes()
            .iter()
            .fold((0 as usize, 0 as i32), |(mut last, mut acc), x| {
                acc = acc + i32::abs(map[(x - b'a') as usize] as i32 - last as i32);
                last = map[(x - b'a') as usize];
                (last, acc)
            })
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1165() {
        assert_eq!(
            Solution::calculate_time("abcdefghijklmnopqrstuvwxyz".to_string(), "cba".to_string()),
            4
        );
        assert_eq!(
            Solution::calculate_time(
                "pqrstuvwxyzabcdefghijklmno".to_string(),
                "leetcode".to_string()
            ),
            73
        );
    }
}
