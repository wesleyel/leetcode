/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
 *
 * https://leetcode.cn/problems/remove-duplicates-from-sorted-list/description/
 *
 * category: algorithms
 * tags: linked-list
 * Easy (54.19%)
 * Likes:    1168
 * Dislikes: 0
 * Total Accepted:    739.7K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。返回 已排序的链表 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：head = [1,1,2]
 * 输出：[1,2]
 *
 *
 * 示例 2：
 *
 *
 * 输入：head = [1,1,2,3,3]
 * 输出：[1,2,3]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 链表中节点数目在范围 [0, 300] 内
 * -100 <= Node.val <= 100
 * 题目数据保证链表已经按升序 排列
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
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut current = head.as_mut().unwrap();
        while let Some(next) = current.next.as_mut() {
            if current.val == next.val {
                current.next = next.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        head
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::delete_duplicates(list_node![1, 1, 2]),
            list_node![1, 2]
        );
        assert_eq!(
            Solution::delete_duplicates(list_node![1, 1, 2, 3, 3]),
            list_node![1, 2, 3]
        );
    }
}
