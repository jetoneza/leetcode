#![allow(dead_code, unused_variables)]

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
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next;

            node.next = prev;

            prev = Some(node);
        }

        prev
    }
}

pub fn create_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut curr: Option<Box<ListNode>> = None;
    let mut reversed_list = list.clone();
    reversed_list.reverse();

    for v in reversed_list.into_iter() {
        let mut new = Box::new(ListNode { val: v, next: None });

        if let Some(node) = curr {
            new.next = Some(node);
            curr = Some(new);
            continue;
        }

        curr = Some(new);
    }

    curr
}

#[cfg(test)]
mod tests {
    use crate::solutions::s023_reverse_linked_list::create_list;
    use crate::solutions::s023_reverse_linked_list::Solution;

    #[test]
    fn it_works() {
        let list = create_list(vec![1, 2, 3, 4, 5]);
        let reversed_list = create_list(vec![5, 4, 3, 2, 1]);

        assert_eq!(Solution::reverse_list(list), reversed_list);
    }
}
