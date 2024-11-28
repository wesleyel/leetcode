/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 *
 * https://leetcode.cn/problems/median-of-two-sorted-arrays/description/
 *
 * category: algorithms
 * tags: array,binary-search,divide-and-conquer
 * Hard (42.75%)
 * Likes:    7270
 * Dislikes: 0
 * Total Accepted:    1.2M
 * Total Submissions: 2.8M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
 *
 * 算法的时间复杂度应该为 O(log (m+n)) 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums1 = [1,3], nums2 = [2]
 * 输出：2.00000
 * 解释：合并数组 = [1,2,3] ，中位数 2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums1 = [1,2], nums2 = [3,4]
 * 输出：2.50000
 * 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
 *
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -10^6 <= nums1[i], nums2[i] <= 10^6
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut nums_a, mut nums_b) = (nums1, nums2);
        if nums_a.len() > nums_b.len() {
            std::mem::swap(&mut nums_a, &mut nums_b);
        }
        let m = nums_a.len();
        let n = nums_b.len();
        let mut low = 0;
        let mut high = m;

        while low <= high {
            // i,j 是 nums_a, nums_b 的切分点
            // 通过二分法找到一个切分点
            let i = (low + high) / 2;
            let j = (m + n + 1) / 2 - i;

            // i_left, i_right, j_left, j_right 分别是 nums_a, nums_b 的切分点左右两边的值
            let i_left = if i == 0 { std::i32::MIN } else { nums_a[i - 1] };
            let i_right = if i == m { std::i32::MAX } else { nums_a[i] };
            let j_left = if j == 0 { std::i32::MIN } else { nums_b[j - 1] };
            let j_right = if j == n { std::i32::MAX } else { nums_b[j] };

            // nums_a[0..i-1] 和 nums_b[0..j-1] 代表合并后的集合的左半部分
            // nums_a[i..] 和 nums_b[j..] 代表合并后的集合的右半部分

            // 由于 nums_a 和 nums_b 都是有序的，所以 i_left <= i_right，j_left <= j_right
            // 只需要判断 i_left <= j_right && j_left <= i_right 是否成立，就可以确定是否找到了合适的切分点

            if i_left <= j_right && j_left <= i_right {
                // 如果 i_left <= j_right && j_left <= i_right，说明找到了合适的切分点
                if (m + n) % 2 == 1 {
                    return (i_left.max(j_left)) as f64;
                } else {
                    return ((i_left.max(j_left)) as f64 + (i_right.min(j_right)) as f64) / 2.0;
                }
            } else if i_left > j_right {
                // 如果 i_left > j_right，说明 i 太大，需要减小 i
                high = i - 1;
            } else {
                // 如果 j_left > i_right，说明 i 太小，需要增大 i
                low = i + 1;
            }
        }
        0.0
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);

        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }
}
