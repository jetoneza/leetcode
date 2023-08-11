#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for num in &nums {
            let count = map.entry(num).or_insert(0);

            *count += 1;

            if *count > 1 {
                return true
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::contains_duplicate::Solution;

    #[test]
    fn it_works() {
        let input_1 = Vec::from([1, 2, 3, 1]);
        let input_2 = Vec::from([1, 2, 3, 4]);
        let input_3 = Vec::from([1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);

        assert_eq!(Solution::contains_duplicate(input_1), true);
        assert_eq!(Solution::contains_duplicate(input_2), false);
        assert_eq!(Solution::contains_duplicate(input_3), true);
    }
}
