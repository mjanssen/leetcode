// task_206 - https://leetcode.com/problems/reverse-linked-list/description/

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

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = &head;
        let mut result: Option<Box<ListNode>> = None;

        while let Some(c) = curr {
            result = match result {
                Some(r) => Some(Box::new(ListNode {
                    val: c.val,
                    next: Some(r),
                })),
                None => Some(Box::new(ListNode {
                    val: c.val,
                    next: None,
                })),
            };

            curr = &c.next;
        }

        result
    }
}
