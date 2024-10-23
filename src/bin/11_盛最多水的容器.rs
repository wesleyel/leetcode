/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 *
 * https://leetcode.cn/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (60.69%)
 * Likes:    5148
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 2.4M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * 给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。
 *
 * 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
 *
 * 返回容器可以储存的最大水量。
 *
 * 说明：你不能倾斜容器。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：[1,8,6,2,5,4,8,3,7]
 * 输出：49
 * 解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
 *
 * 示例 2：
 *
 *
 * 输入：height = [1,1]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        fn calc_area(height: &Vec<i32>, left: usize, right: usize) -> i32 {
            let h = height[left].min(height[right]);
            let w = (right - left) as i32;
            h * w
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = calc_area(&height, left, right);
        while left < right {
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
            max_area = max_area.max(calc_area(&height, left, right));
        }
        max_area
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);

        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
