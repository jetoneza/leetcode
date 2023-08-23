#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

use super::s026_invert_binary_tree::{Tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // See implementation of Tree::compare
        Tree::compare(&p, &q)
    }
}

fn create_test_tree() -> Option<Rc<RefCell<TreeNode>>> {
    fn create_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }

    Some(create_node(1, None, Some(create_node(2, None, None))))
}

#[cfg(test)]
mod tests {
    use crate::solutions::s026_invert_binary_tree::Tree;
    use crate::solutions::s030_same_tree::{create_test_tree, Solution};

    #[test]
    fn it_works() {
        let tree_1 = Tree::create(vec![1, 2, 3]);
        let tree_2 = Tree::create(vec![1, 2, 3]);
        let tree_3 = Tree::create(vec![1, 2]);
        let tree_4 = create_test_tree();

        assert_eq!(Solution::is_same_tree(tree_1, tree_2), true);
        assert_eq!(Solution::is_same_tree(tree_3, tree_4), false);
    }
}
