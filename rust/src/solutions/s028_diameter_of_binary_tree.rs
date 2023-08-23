#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

use super::s026_invert_binary_tree::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut d = 0;

        Solution::walk(&root, &mut d);

        d
    }

    fn walk(curr: &Option<Rc<RefCell<TreeNode>>>, d: &mut i32) -> i32 {
        match curr {
            Some(l) => {
                let left = &l.borrow().left;
                let right = &l.borrow().right;

                let ld = Solution::walk(left, d);
                let rd = Solution::walk(right, d);

                *d = std::cmp::max(*d, ld + rd);

                std::cmp::max(ld, rd) + 1
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s026_invert_binary_tree::Tree;
    use crate::solutions::s028_diameter_of_binary_tree::Solution;

    #[test]
    fn it_works() {
        let input = vec![1, 2, 3, 4, 5];
        let input_2 = vec![1];

        assert_eq!(Solution::diameter_of_binary_tree(Tree::create(input)), 3);
        assert_eq!(Solution::diameter_of_binary_tree(Tree::create(input_2)), 0);
    }
}
