/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 *
 * https://leetcode.cn/problems/minimum-window-substring/description/
 *
 * category: algorithms
 * tags: hash-table,two-pointers,string,sliding-window
 * Hard (47.30%)
 * Likes:    3206
 * Dislikes: 0
 * Total Accepted:    758.4K
 * Total Submissions: 1.6M
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 ""
 * 。
 *
 *
 *
 * 注意：
 *
 *
 * 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
 * 如果 s 中存在这样的子串，我们保证它是唯一的答案。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "ADOBECODEBANC", t = "ABC"
 * 输出："BANC"
 * 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "a", t = "a"
 * 输出："a"
 * 解释：整个字符串 s 是最小覆盖子串。
 *
 *
 * 示例 3:
 *
 *
 * 输入: s = "a", t = "aa"
 * 输出: ""
 * 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
 * 因此没有符合条件的子字符串，返回空字符串。
 *
 *
 *
 * 提示：
 *
 *
 * ^m == s.length
 * ^n == t.length
 * 1 <= m, n <= 10^5
 * s 和 t 由英文字母组成
 *
 *
 *
 * 进阶：你能设计一个在 o(m+n) 时间内解决此问题的算法吗？
 */

struct Solution;
fn main() {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

macro_rules! list_node {
    () => {None};
    ($($e:expr), *) => {
        {
            // create a template head
            let mut temp_head = Box::new(ListNode::new(0));
            // use ref_head as a mutable reference to the last node
            let mut _ref_head = &mut temp_head;
            $(
                // append a new node to the last node
                _ref_head.next = Some(Box::new(ListNode::new($e)));
                _ref_head = _ref_head.next.as_mut().unwrap();
            )*
            // return the head of the list (skip the template head)
            Some(temp_head.next.unwrap())
        }
    };
}

// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;

        let s_chars: Vec<char> = s.chars().collect();

        // 如果s比t短，不可能包含所有t中的字符
        if s.len() < t.len() {
            return "".to_string();
        }

        // 统计t中字符出现的次数
        let mut t_counter: HashMap<char, i32> = HashMap::new();
        for ch in t.chars() {
            *t_counter.entry(ch).or_insert(0) += 1;
        }

        let mut window_counter: HashMap<char, i32> = HashMap::new();
        let required = t_counter.len(); // 需要满足的不同字符数
        let mut formed = 0; // 已经满足要求的不同字符数

        let mut ans = (usize::MAX, 0, 0); // (窗口长度, 左边界, 右边界)

        // 滑动窗口: [left, right)
        let mut left = 0;
        let mut right = 0;

        while right < s_chars.len() {
            // 扩大窗口
            let c = s_chars[right];
            *window_counter.entry(c).or_insert(0) += 1;

            // 检查当前字符是否在t中，且出现次数是否达到要求
            if t_counter.contains_key(&c) && window_counter.get(&c) == t_counter.get(&c) {
                formed += 1;
            }

            // 当窗口包含t中所有需要的字符时，尝试缩小窗口
            while left <= right && formed == required {
                // 更新最小窗口
                if right - left < ans.0 {
                    ans = (right - left, left, right);
                }

                // 缩小窗口
                let c = s_chars[left];
                *window_counter.get_mut(&c).unwrap() -= 1;

                // 检查移除后是否不再满足要求
                if t_counter.contains_key(&c) && window_counter.get(&c) < t_counter.get(&c) {
                    formed -= 1;
                }

                left += 1;
            }

            right += 1;
        }

        if ans.0 == usize::MAX {
            "".to_string()
        } else {
            s_chars[ans.1..=ans.2].iter().collect()
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let res = Solution::min_window(s, t);
        assert_eq!(res, "BANC".to_string());
    }

    #[test]
    fn test2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let res = Solution::min_window(s, t);
        assert_eq!(res, "a".to_string());
    }

    #[test]
    fn test3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let res = Solution::min_window(s, t);
        assert_eq!(res, "".to_string());
    }
}
