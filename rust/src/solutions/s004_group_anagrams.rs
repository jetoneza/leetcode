#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for word in &strs {
            let sorted_word = sort(word);

            if let Some(anagrams) = map.get_mut(&sorted_word) {
                anagrams.push(word.to_owned());
                continue;
            }

            map.insert(sorted_word, Vec::from([word.to_owned()]));
        }

        let mut group_anagrams = Vec::new();

        map.iter().for_each(|(key, anagrams)| {
            group_anagrams.push(anagrams.to_owned());
        });

        group_anagrams
    }
}

fn sort(s: &String) -> String {
    let mut word: Vec<char> = s.chars().collect();

    word.sort();

    word.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::solutions::s004_group_anagrams::Solution;

    #[test]
    fn it_works() {
        let mut group = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);

        group.sort();

        assert_eq!(
            group,
            vec![
                vec!["bat".to_string()],
                vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
                vec!["tan".to_string(), "nat".to_string()],
            ]
        );
    }
}
