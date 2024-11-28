/*
 * @lc app=leetcode.cn id=61 lang=rust
 *
 * [61] 旋转链表
 *
 * https://leetcode.cn/problems/rotate-list/description/
 *
 * category: algorithms
 * tags: linked-list,two-pointers
 * Medium (41.45%)
 * Likes:    1100
 * Dislikes: 0
 * Total Accepted:    421.8K
 * Total Submissions: 1M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你一个链表的头节点 head ，旋转链表，将链表每个节点向右移动 k 个位置。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：head = [1,2,3,4,5], k = 2
 * 输出：[4,5,1,2,3]
 *
 *
 * 示例 2：
 *
 *
 * 输入：head = [0,1,2], k = 4
 * 输出：[2,0,1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 链表中节点的数目在范围 [0, 500] 内
 * -100 <= Node.val <= 100
 * 0 <= k <= 2 * 10^9
 *
 *
 */

struct Solution;
fn main() {}

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 || head.as_ref().is_none() {
            return head;
        }
        let mut node = head.as_ref();
        let mut len = 0;
        while let Some(next) = node {
            len += 1;
            node = next.next.as_ref();
        }

        let k = k % len;
        if k == 0 {
            return head;
        }

        let mut head = head;
        // head: 1->2->3->4->5  k=2
        let mut mid = head.as_mut();
        for _ in 0..len - k - 1 {
            mid = mid.unwrap().next.as_mut();
        }
        // mid: 3
        let mut head_end = mid.as_mut().unwrap().next.take();
        // head_end: 4->5  head 1->2->3
        let mut head_mut = head_end.as_mut();
        while head_mut.as_ref().unwrap().next.is_some() {
            head_mut = head_mut.unwrap().next.as_mut();
        }
        // head_mut: 5
        head_mut.unwrap().next = head.take();
        // head_mut 5->1->2->3
        head_end
        // 4->5->1->2->3
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::rotate_right(list_node![1, 2, 3, 4, 5], 2),
            list_node![4, 5, 1, 2, 3]
        );
        assert_eq!(
            Solution::rotate_right(list_node![0, 1, 2], 4),
            list_node![2, 0, 1]
        );
    }
}
