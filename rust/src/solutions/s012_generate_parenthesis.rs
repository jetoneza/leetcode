#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        compose(0, 0, n, String::from(""), &mut result);

        result
    }
}

fn compose(open: i32, close: i32, n: i32, parens: String, result: &mut Vec<String>) {
    if open == close && close == n {
        result.push(parens);
        return;
    }

    if open < n {
        compose(open + 1, close, n, parens.to_owned() + "(", result);
    }

    if close < open {
        compose(open, close + 1, n, parens.to_owned() + ")", result);
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s012_generate_parenthesis::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ],
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_string(),],);
    }
}
