/*
 * @lc app=leetcode.cn id=2182 lang=rust
 *
 * [2182] 构造限制重复的字符串
 *
 * https://leetcode.cn/problems/construct-string-with-repeat-limit/description/
 *
 * algorithms
 * Medium (60.40%)
 * Likes:    86
 * Dislikes: 0
 * Total Accepted:    23.2K
 * Total Submissions: 38.5K
 * Testcase Example:  '"cczazcc"\n3'
 *
 * 给你一个字符串 s 和一个整数 repeatLimit ，用 s 中的字符构造一个新字符串 repeatLimitedString ，使任何字母 连续
 * 出现的次数都不超过 repeatLimit 次。你不必使用 s 中的全部字符。
 *
 * 返回 字典序最大的 repeatLimitedString 。
 *
 * 如果在字符串 a 和 b 不同的第一个位置，字符串 a 中的字母在字母表中出现时间比字符串 b 对应的字母晚，则认为字符串 a 比字符串 b 字典序更大
 * 。如果字符串中前 min(a.length, b.length) 个字符都相同，那么较长的字符串字典序更大。
 *
 *
 *
 * 示例 1：
 *
 * 输入：s = "cczazcc", repeatLimit = 3
 * 输出："zzcccac"
 * 解释：使用 s 中的所有字符来构造 repeatLimitedString "zzcccac"。
 * 字母 'a' 连续出现至多 1 次。
 * 字母 'c' 连续出现至多 3 次。
 * 字母 'z' 连续出现至多 2 次。
 * 因此，没有字母连续出现超过 repeatLimit 次，字符串是一个有效的 repeatLimitedString 。
 * 该字符串是字典序最大的 repeatLimitedString ，所以返回 "zzcccac" 。
 * 注意，尽管 "zzcccca" 字典序更大，但字母 'c' 连续出现超过 3 次，所以它不是一个有效的 repeatLimitedString 。
 *
 *
 * 示例 2：
 *
 * 输入：s = "aababab", repeatLimit = 2
 * 输出："bbabaa"
 * 解释：
 * 使用 s 中的一些字符来构造 repeatLimitedString "bbabaa"。
 * 字母 'a' 连续出现至多 2 次。
 * 字母 'b' 连续出现至多 2 次。
 * 因此，没有字母连续出现超过 repeatLimit 次，字符串是一个有效的 repeatLimitedString 。
 * 该字符串是字典序最大的 repeatLimitedString ，所以返回 "bbabaa" 。
 * 注意，尽管 "bbabaaa" 字典序更大，但字母 'a' 连续出现超过 2 次，所以它不是一个有效的 repeatLimitedString
 * 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= repeatLimit <= s.length <= 10^5
 * s 由小写英文字母组成
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut bitmap = vec![0; 26];
        s.bytes().for_each(|c| {
            bitmap[(c - b'a') as usize] += 1;
        });
        let mut ans = String::new();

        let (mut cur_i, mut next_i) = (25, 24);
        let mut repeat = 0;
        while cur_i < 26 && next_i < 25 {
            if bitmap[cur_i] == 0 {
                // cur 不存在，查找下一个
                repeat = 0;
                match cur_i.checked_sub(1) {
                    Some(i) => cur_i = i,
                    None => break,
                }
            } else if repeat < repeat_limit {
                // cur 存在，且未超过长度限制
                ans.push((b'a' + cur_i as u8) as char);
                bitmap[cur_i] -= 1;
                repeat += 1;
            } else if next_i >= cur_i || bitmap[next_i] == 0 {
                // next 不存在，查找下一个
                match next_i.checked_sub(1) {
                    Some(i) => next_i = i,
                    None => break,
                }
            } else {
                // next 存在，且未超过长度限制
                ans.push((b'a' + next_i as u8) as char);
                bitmap[next_i] -= 1;
                repeat = 0;
            }
        }
        ans
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "cczazcc".to_string();
        let repeat_limit = 3;
        assert_eq!(
            Solution::repeat_limited_string(s, repeat_limit),
            "zzcccac".to_string()
        );

        let s = "aababab".to_string();
        let repeat_limit = 2;
        assert_eq!(
            Solution::repeat_limited_string(s, repeat_limit),
            "bbabaa".to_string()
        );

        let s = "bplpcfifosybmjxphbxdltxtfrjspgixoxzbpwrtkopepjxfooazjyosengdlvyfchqhqxznnhuuxhtbrojyhxwlsrklsryvmufoibgfyxgjw".to_string();
        let repeat_limit = 1;
        assert_eq!(
            Solution::repeat_limited_string(s, repeat_limit),
            "zyzyzyxyxyxyxwxwxwxvxvxuxututststsrsrsrqrqrpopopopopopopononmnmlklkljljljijijijhghghghghfhfefefdfdfcfcbab".to_string()
        );
    }
}
