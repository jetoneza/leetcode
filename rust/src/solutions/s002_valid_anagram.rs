#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let first = sort(&s);
        let second = sort(&t);

        first == second
    }
}

fn sort(s: &String) -> String {
    let mut word: Vec<char> = s.chars().collect();

    word.sort();

    word.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::solutions::s002_valid_anagram::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }
}
