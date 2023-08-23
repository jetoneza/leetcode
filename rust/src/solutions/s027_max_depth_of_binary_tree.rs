#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

use super::s026_invert_binary_tree::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::walk(&root, 0)
    }

    fn walk(curr: &Option<Rc<RefCell<TreeNode>>>, h: i32) -> i32 {
        match curr {
            Some(l) => {
                let lh = Solution::walk(&l.borrow().left, h + 1);
                let rh = Solution::walk(&l.borrow().right, h + 1);

                std::cmp::max(lh, rh)
            }
            None => h,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s026_invert_binary_tree::Tree;
    use crate::solutions::s027_max_depth_of_binary_tree::Solution;

    #[test]
    fn it_works() {
        let input = vec![3, 9, 20, 15, 7];

        assert_eq!(Solution::max_depth(Tree::create(input)), 3);
    }
}
