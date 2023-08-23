#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

use super::s026_invert_binary_tree::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::walk(&root).is_some()
    }

    fn walk(curr: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match curr {
            Some(n) => {
                let left = Solution::walk(&n.borrow().left)?;
                let right = Solution::walk(&n.borrow().right)?;

                let diff = (left - right).abs();

                if diff > 1 {
                    return None;
                }

                Some(std::cmp::max(left, right) + 1)
            }
            None => Some(0),
        }
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

    Some(create_node(
        3,
        Some(create_node(9, None, None)),
        Some(create_node(
            20,
            Some(create_node(15, None, None)),
            Some(create_node(7, None, None)),
        )),
    ))
}

#[cfg(test)]
mod tests {
    use crate::solutions::s029_balanced_binary_tree::{create_test_tree, Solution};

    #[test]
    fn it_works() {
        let tree = create_test_tree();

        assert_eq!(Solution::is_balanced(tree), true);
    }
}
