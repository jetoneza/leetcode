#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();

        if len == 1 {
            return 0;
        }

        let mut lmax = height[0];
        let mut rmax = height[len - 1];
        let mut l = 1;
        let mut r = len - 2;
        let mut ans = 0;

        while l <= r {
            if height[l] >= lmax {
                lmax = height[l];
                l += 1;
            } else if height[r] >= rmax {
                rmax = height[r];
                r -= 1;
            } else if lmax <= rmax && height[l] < lmax {
                ans += lmax - height[l];

                l += 1;
            } else {
                ans += rmax - height[r];

                r -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s020_trapping_rain_water::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(Solution::trap(vec![2, 0, 2]), 2);
        assert_eq!(Solution::trap(vec![0]), 0);
    }
}
