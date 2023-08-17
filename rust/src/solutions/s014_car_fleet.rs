#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut fleets = vec![];
        let mut cars: Vec<(i32, i32)> = position
            .into_iter()
            .zip(speed.into_iter())
            .collect();

        cars.sort_unstable();

        for i in (0..cars.len()).rev() {
            let (pos, speed) = cars[i];
            let t = (target - pos) as f32 / speed as f32;

            fleets.push(t);

            let len = fleets.len();

            if len >= 2 && fleets[len - 1] <= fleets[len - 2] {
                fleets.pop();
            }
        }

        fleets.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s014_car_fleet::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
        assert_eq!(Solution::car_fleet(10, vec![6, 8], vec![3, 2]), 2);
    }
}
