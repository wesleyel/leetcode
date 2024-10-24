/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 *
 * https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (49.51%)
 * Likes:    2958
 * Dislikes: 0
 * Total Accepted:    1.6M
 * Total Submissions: 3.2M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：head = [1,2,3,4,5], n = 2
 * 输出：[1,2,3,5]
 *
 *
 * 示例 2：
 *
 *
 * 输入：head = [1], n = 1
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：head = [1,2], n = 1
 * 输出：[1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 链表中结点的数目为 sz
 * 1 <= sz <= 30
 * 0 <= Node.val <= 100
 * 1 <= n <= sz
 *
 *
 *
 *
 * 进阶：你能尝试使用一趟扫描实现吗？
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
// Accepted
// 208/208 cases passed (1 ms)
// Your runtime beats 68.18 % of rust submissions
// Your memory usage beats 10.91 % of rust submissions (2.1 MB)
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 快慢指针
        // 偏移快指针n个位置，然后快慢指针一起移动，当快指针到达末尾时，慢指针指向的就是要删除的节点
        match head {
            None => None,
            Some(mut head) => {
                let mut fast = &mut head.clone();
                let mut slow = &mut head;
                for _ in 0..n {
                    if let Some(f) = fast.next.as_mut() {
                        fast = f;
                    } else {
                        return head.next;
                    }
                }
                while let Some(f) = fast.next.as_mut() {
                    fast = f;
                    slow = slow.next.as_mut().unwrap();
                }
                slow.next = slow.next.as_mut().unwrap().next.take();
                Some(head)
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut p = head.as_mut();
        for i in vec {
            if let Some(node) = p {
                node.next = Some(Box::new(ListNode::new(i)));
                p = node.next.as_mut();
            }
        }
        head.unwrap().next
    }

    #[test]
    fn test() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        let res = to_list(vec![1, 2, 3, 5]);
        assert_eq!(Solution::remove_nth_from_end(head, 2), res);

        let head = to_list(vec![1]);
        let res = to_list(vec![]);
        assert_eq!(Solution::remove_nth_from_end(head, 1), res);

        let head = to_list(vec![1, 2]);
        let res = to_list(vec![1]);
        assert_eq!(Solution::remove_nth_from_end(head, 1), res);
    }
}
