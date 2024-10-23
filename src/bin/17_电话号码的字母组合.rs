/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (60.78%)
 * Likes:    2925
 * Dislikes: 0
 * Total Accepted:    971.3K
 * Total Submissions: 1.6M
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
 *
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：digits = "23"
 * 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：digits = ""
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：digits = "2"
 * 输出：["a","b","c"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= digits.length <= 4
 * digits[i] 是范围 ['2', '9'] 的一个数字。
 *
 *
 */

struct Solution;
fn main() {}

// Accepted
// 25/25 cases passed (0 ms)
// Your runtime beats 100 % of rust submissions
// Your memory usage beats 83.52 % of rust submissions (2 MB)

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let hashmap = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];

        digits
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize - 2)
            .map(|i| hashmap[i].clone())
            .fold(vec![String::new()], |acc, x| {
                acc.into_iter()
                    .flat_map(|s| x.iter().map(move |&c| s.clone() + &c.to_string()))
                    .collect()
            })
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let digits = "23".to_string();
        let res = Solution::letter_combinations(digits);
        let ans = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(res, ans);

        let digits = "".to_string();
        let res = Solution::letter_combinations(digits);
        let ans: Vec<&str> = vec![];
        assert_eq!(res, ans);

        let digits = "2".to_string();
        let res = Solution::letter_combinations(digits);
        let ans = vec!["a", "b", "c"];
        assert_eq!(res, ans);
    }
}
