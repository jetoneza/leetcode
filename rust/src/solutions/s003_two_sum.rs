#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let remainder = target - num;

            if let Some(map_index) = map.get(&remainder) {
                return Vec::from([map_index.to_owned(), index.to_owned() as i32]);
            }

            map.insert(num.to_owned(), index as i32);
        }

        Vec::from([])
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s003_two_sum::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::two_sum(Vec::from([2, 7, 11, 15]), 9),
            Vec::from([0, 1])
        );
        assert_eq!(
            Solution::two_sum(Vec::from([3, 2, 4]), 6),
            Vec::from([1, 2])
        );
        assert_eq!(Solution::two_sum(Vec::from([3, 3]), 6), Vec::from([0, 1]));
    }
}
