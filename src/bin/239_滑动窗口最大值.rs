/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 *
 * https://leetcode.cn/problems/sliding-window-maximum/description/
 *
 * category: algorithms
 * tags: heap,sliding-window
 * Hard (49.43%)
 * Likes:    3090
 * Dislikes: 0
 * Total Accepted:    816.7K
 * Total Submissions: 1.7M
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k
 * 个数字。滑动窗口每次只向右移动一位。
 *
 * 返回 滑动窗口中的最大值 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
 * 输出：[3,3,5,5,6,7]
 * 解释：
 * 滑动窗口的位置                最大值
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 * ⁠1 [3  -1  -3] 5  3  6  7       3
 * ⁠1  3 [-1  -3  5] 3  6  7       5
 * ⁠1  3  -1 [-3  5  3] 6  7       5
 * ⁠1  3  -1  -3 [5  3  6] 7       6
 * ⁠1  3  -1  -3  5 [3  6  7]      7
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1], k = 1
 * 输出：[1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * 1 <= k <= nums.length
 *
 *
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
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len() + 1 - k as usize);
        let mut deq = VecDeque::new();

        for i in 0..nums.len() {
            while deq.back().is_some_and(|&j| nums[j] <= nums[i]) {
                deq.pop_back();
            }

            deq.push_back(i);

            if i == deq[0] + k as usize {
                deq.pop_front();
            }

            if i >= k as usize - 1 {
                res.push(nums[deq[0]])
            }
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let res = Solution::max_sliding_window(nums, k);
        assert_eq!(res, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn test2() {
        let nums = vec![1];
        let k = 1;
        let res = Solution::max_sliding_window(nums, k);
        assert_eq!(res, vec![1]);
    }
}
