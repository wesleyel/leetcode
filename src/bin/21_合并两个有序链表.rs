/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 *
 * https://leetcode.cn/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (67.14%)
 * Likes:    3612
 * Dislikes: 0
 * Total Accepted:    1.9M
 * Total Submissions: 2.8M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：l1 = [1,2,4], l2 = [1,3,4]
 * 输出：[1,1,2,3,4,4]
 *
 *
 * 示例 2：
 *
 *
 * 输入：l1 = [], l2 = []
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：l1 = [], l2 = [0]
 * 输出：[0]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 两个链表的节点数目范围是 [0, 50]
 * -100
 * l1 和 l2 均按 非递减顺序 排列
 *
 *
 */

// Definition for singly-linked list.
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
struct Solution;

// @lc code=start
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut l1) = list1 {
            if let Some(mut l2) = list2 {
                if l1.val < l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    return Some(l1);
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    return Some(l2);
                }
            } else {
                return Some(l1);
            }
        } else {
            return list2;
        }
    }
}
// @lc code=end

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_list(list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut p = head.as_mut();
        for i in list {
            if let Some(node) = p {
                node.next = Some(Box::new(ListNode::new(i)));
                p = node.next.as_mut();
            }
        }
        head.unwrap().next
    }

    #[test]
    fn test() {
        let l1 = init_list(vec![1, 2, 4]);
        let l2 = init_list(vec![1, 3, 4]);
        let res = vec![1, 1, 2, 3, 4, 4];
        let mut p = Solution::merge_two_lists(l1, l2);
        for i in res {
            if let Some(node) = p {
                assert_eq!(node.val, i);
                p = node.next;
            }
        }
    }
}
