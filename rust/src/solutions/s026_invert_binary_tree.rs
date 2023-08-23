#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::swap(&root);

        root
    }

    fn swap(curr: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = curr {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            node.borrow_mut().right = left;
            node.borrow_mut().left = right;

            Solution::swap(&node.borrow().left);
            Solution::swap(&node.borrow().right);
        }
    }
}

pub struct Tree {}

impl Tree {
    pub fn create(input: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;

        for &i in input.iter() {
            root = Tree::insert_to_tree(root, i);
        }

        root
    }

    fn insert_to_tree(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })));

        if root.is_none() {
            root = node;
            return root;
        }

        let mut q = vec![root.to_owned()];

        while !q.is_empty() {
            let curr = q.remove(0);

            if let Some(n) = curr {
                if n.borrow().left.is_none() {
                    n.borrow_mut().left = node;
                    return root;
                } else if n.borrow().right.is_none() {
                    n.borrow_mut().right = node;
                    return root;
                }

                q.push(n.borrow().left.clone());
                q.push(n.borrow().right.clone());
            }
        }

        root
    }

    // Used for test and is solution to s30_same_tree
    pub fn compare(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(a), Some(b)) => {
                if a.borrow().val != b.borrow().val {
                    return false;
                }

                return Tree::compare(&a.borrow().left, &b.borrow().left)
                    && Tree::compare(&a.borrow().right, &b.borrow().right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s026_invert_binary_tree::Solution;
    use crate::solutions::s026_invert_binary_tree::Tree;

    #[test]
    fn it_works() {
        let input = vec![4, 2, 7, 1, 3, 6, 9];
        let reversed_input = vec![4, 7, 2, 9, 6, 3, 1];

        let root = Solution::invert_tree(Tree::create(input));
        let reverse = Tree::create(reversed_input);

        assert_eq!(Tree::compare(&root, &reverse), true);
    }
}
