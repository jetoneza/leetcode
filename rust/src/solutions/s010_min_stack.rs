#![allow(dead_code, unused_variables)]

pub struct MinStack {
    min: Vec<i32>,
    arr: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            min: Vec::new(),
            arr: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.arr.push(val);

        if self.min.is_empty() {
            self.min.push(val);
            return;
        }

        match self.min.last() {
            Some(min) => {
                let to_push = if val <= min.to_owned() {
                    val
                } else {
                    min.to_owned()
                };

                self.min.push(to_push)
            }
            None => {}
        };
    }

    fn pop(&mut self) {
        self.arr.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        match self.arr.last() {
            Some(top) => top.to_owned(),
            None => 0,
        }
    }

    fn get_min(&self) -> i32 {
        match self.min.last() {
            Some(min) => min.to_owned(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s010_min_stack::MinStack;

    #[test]
    fn it_works() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);

        let first_min = stack.get_min();

        stack.pop();

        let top = stack.top();
        let second_min = stack.get_min();

        assert_eq!(first_min, -3);
        assert_eq!(top, 0);
        assert_eq!(second_min, -2);
    }
}
