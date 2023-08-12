#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: HashMap<u8, Vec<char>> = HashMap::new();
        let mut cols: HashMap<u8, Vec<char>> = HashMap::new();
        let mut square: HashMap<(u8, u8), Vec<char>> = HashMap::new();

        for r in 0..9 {
            for c in 0..9 {
                let value = board[r][c];

                if value == '.' {
                    continue;
                }

                let row = rows.entry(r as u8).or_insert(vec![]);

                if row.contains(&value) {
                    return false;
                }

                let col = cols.entry(c as u8).or_insert(vec![]);

                if col.contains(&value) {
                    return false;
                }

                let sq = square
                    .entry(((r / 3) as u8, (c / 3) as u8))
                    .or_insert(vec![]);

                if sq.contains(&value) {
                    return false;
                }

                row.push(value);
                col.push(value);
                sq.push(value);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s007_valid_sudoku::Solution;

    #[test]
    fn it_works() {
        let input_1 = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let input_2 = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(input_1), true);
        assert_eq!(Solution::is_valid_sudoku(input_2), false);
    }
}
