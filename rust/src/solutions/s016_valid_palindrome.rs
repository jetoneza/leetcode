#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let word: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let len = word.len();

        for begin in 0..len {
            let end = len - 1 - begin;

            let c_begin = word[begin];
            let c_end = word[end];

            if c_begin != c_end {
                return false;
            }

            if begin == end {
                break;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s016_valid_palindrome::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true,
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false,);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true,);
    }
}
