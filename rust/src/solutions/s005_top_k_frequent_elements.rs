#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        nums.iter().for_each(|num| {
            let count = map.entry(num.to_owned()).or_insert(0);

            *count += 1;
        });

        let mut sorted_pairs: Vec<(i32, i32)> =
            map.iter().map(|(&key, &value)| (key, value)).collect();

        sorted_pairs.sort_by(|first, second| second.1.cmp(&first.1));

        sorted_pairs
            .into_iter()
            .take(k as usize)
            .map(|(key, _)| key)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s005_top_k_frequent_elements::Solution;

    #[test]
    fn it_works() {
        let input_1 = Vec::from([1, 1, 1, 2, 2, 3]);
        let input_2 = Vec::from([1]);
        let input_3 = Vec::from([1, 2]);

        let mut input_1_frequent = Solution::top_k_frequent(input_1, 2);
        input_1_frequent.sort();

        let input_2_frequent = Solution::top_k_frequent(input_2, 1);

        let mut input_3_frequent = Solution::top_k_frequent(input_3, 2);
        input_3_frequent.sort();

        assert_eq!(input_1_frequent, vec![1, 2]);
        assert_eq!(input_2_frequent, vec![1]);
        assert_eq!(input_3_frequent, vec![1, 2]);
    }
}
