/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 *
 * https://leetcode.cn/problems/search-a-2d-matrix/description/
 *
 * algorithms
 * Medium (50.41%)
 * Likes:    980
 * Dislikes: 0
 * Total Accepted:    471.9K
 * Total Submissions: 936.1K
 * Testcase Example:  '[[1,3,5,7],[10,11,16,20],[23,30,34,60]]\n3'
 *
 * 给你一个满足下述两条属性的 m x n 整数矩阵：
 *
 *
 * 每行中的整数从左到右按非严格递增顺序排列。
 * 每行的第一个整数大于前一行的最后一个整数。
 *
 *
 * 给你一个整数 target ，如果 target 在矩阵中，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1 <= m, n <= 100
 * -10^4 <= matrix[i][j], target <= 10^4
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    // 2次二分查找调库
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let y = matrix.binary_search_by_key(&target, |row| row[0]);
        match y {
            // target 在矩阵的第一列
            Ok(_) => true,
            Err(y) => {
                if y == 0 {
                    // target 小于矩阵的第一列
                    return false;
                }
                // target 在矩阵的第 y 行
                let x = matrix[y - 1].binary_search(&target);
                x.is_ok()
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix.clone(), 3), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 13), false);
    }
}
