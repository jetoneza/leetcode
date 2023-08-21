#![allow(dead_code, unused_variables)]

use super::s023_reverse_linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(l1), Some(l2)) => {
                if l1.val >= l2.val {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Solution::merge_two_lists(Some(l1), l2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Solution::merge_two_lists(Some(l2), l1.next),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s023_reverse_linked_list::create_list;
    use crate::solutions::s024_merged_two_sorted_lists::Solution;

    #[test]
    fn it_works() {
        let list1 = create_list(vec![1, 2, 4]);
        let list2 = create_list(vec![1, 3, 4]);

        let ans = create_list(vec![1, 1, 2, 3, 4, 4]);

        assert_eq!(Solution::merge_two_lists(list1, list2), ans);
    }
}
