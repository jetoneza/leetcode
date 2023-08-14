#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            if stack.len() == 0 {
                stack.push(c);
                continue;
            }

            let Some(stack_char) = stack.last() else {
                continue;
            };

            match (stack_char.to_owned(), c) {
                ('(', ')') | ('[', ']') | ('{', '}') => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s009_valid_parenthesis::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
}
