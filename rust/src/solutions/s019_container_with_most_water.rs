#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max = 0;

        let mut l = 0;
        let mut r = heights.len() - 1;

        while l < r {
            let lh = heights[l];
            let rh = heights[r];

            let a = std::cmp::min(lh, rh) * (r - l) as i32;

            max = std::cmp::max(a, max);

            if lh < rh {
                l += 1;
            } else {
                r -= 1;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s019_container_with_most_water::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
