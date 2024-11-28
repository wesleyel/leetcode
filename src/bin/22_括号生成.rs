/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 *
 * https://leetcode.cn/problems/generate-parentheses/description/
 *
 * category: algorithms
 * tags: string,backtracking
 * Medium (78.20%)
 * Likes:    3713
 * Dislikes: 0
 * Total Accepted:    927K
 * Total Submissions: 1.2M
 * Testcase Example:  '3'
 *
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3
 * 输出：["((()))","(()())","(())()","()(())","()()()"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：["()"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 8
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        /// dfs
        /// left: 左括号数量
        /// right: 右括号数量
        /// path: 当前路径
        /// res: 结果
        fn dfs(n: i32, left: i32, right: i32, path: &mut String, res: &mut Vec<String>) {
            if left == n && right == n {
                // 左右括号数量都为n时，说明找到了一个有效的括号组合
                res.push(path.clone());
                return;
            }
            if left < n {
                // 左括号数量小于n时，可以添加左括号
                path.push('(');
                dfs(n, left + 1, right, path, res);
                path.pop();
            }
            if right < left {
                // 右括号数量小于左括号数量时，可以添加右括号
                path.push(')');
                dfs(n, left, right + 1, path, res);
                path.pop();
            }
        }
        let mut res = vec![];
        dfs(n, 0, 0, &mut String::new(), &mut res);
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let res = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(Solution::generate_parenthesis(n), res);

        let n = 1;
        let res = vec!["()"];
        assert_eq!(Solution::generate_parenthesis(n), res);
    }
}
