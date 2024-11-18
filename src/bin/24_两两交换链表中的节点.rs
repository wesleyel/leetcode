/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 *
 * https://leetcode.cn/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (73.24%)
 * Likes:    2310
 * Dislikes: 0
 * Total Accepted:    975.8K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：head = [1,2,3,4]
 * 输出：[2,1,4,3]
 *
 *
 * 示例 2：
 *
 *
 * 输入：head = []
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：head = [1]
 * 输出：[1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 链表中节点的数目在范围 [0, 100] 内
 * 0 <= Node.val <= 100
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

// @lc code=start

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut head| {
            if let Some(mut next) = head.next.take() {
                head.next = Self::swap_pairs(next.next.take());
                next.next = Some(head);
                Some(next)
            } else {
                Some(head)
            }
        })
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! list {
        () => {
            None
        };
        ($($e:expr), *) => {
            {
                let mut head = Box::new(ListNode::new(0));
                let mut ref_head = &mut head;
                $(
                    ref_head.next = Some(Box::new(ListNode::new($e)));
                    ref_head = ref_head.next.as_mut().unwrap();
                )*
                Some(head.next.unwrap())
            }
        };
    }

    #[test]
    fn test() {
        assert_eq!(Solution::swap_pairs(list![1, 2, 3, 4]), list![2, 1, 4, 3]);
        assert_eq!(Solution::swap_pairs(list![]), list![]);
        assert_eq!(Solution::swap_pairs(list![1]), list![1]);
    }
}
