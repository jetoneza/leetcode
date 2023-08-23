#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

use super::s026_invert_binary_tree::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_subtree(
        tree: Option<Rc<RefCell<TreeNode>>>,
        subtree: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::compare(&tree, &subtree)
    }

    pub fn compare(
        tree: &Option<Rc<RefCell<TreeNode>>>,
        subtree: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if tree == subtree {
            return true;
        }

        match tree {
            None => false,
            Some(n) => {
                Solution::compare(&n.borrow().left, subtree)
                    || Solution::compare(&n.borrow().right, subtree)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s026_invert_binary_tree::Tree;
    use crate::solutions::s031_subtree_of_another_tree::Solution;

    #[test]
    fn it_works() {
        let tree = Tree::create(vec![3, 4, 5, 1, 2]);
        let sub_tree = Tree::create(vec![4, 1, 2]);

        let tree_2 = Tree::create(vec![1, 1]);
        let sub_tree_2 = Tree::create(vec![1]);

        assert_eq!(Solution::is_subtree(tree, sub_tree), true);
        assert_eq!(Solution::is_subtree(tree_2, sub_tree_2), true);
    }
}
