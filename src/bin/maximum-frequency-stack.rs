use std::collections::HashMap;

#[derive(Default)]
struct FreqStack {
    map: HashMap<i32, usize>,
    stack: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        let k = self.map.entry(val).or_default();
        if *k < self.stack.len() {
            self.stack[*k].push(val);
        } else {
            self.stack.push(vec![val]);
        }
        *k += 1;
    }

    fn pop(&mut self) -> i32 {
        let mut last = self.stack.pop().unwrap();
        let val = last.pop().unwrap();
        *self.map.get_mut(&val).unwrap() -= 1;
        if !last.is_empty() {
            self.stack.push(last);
        }
        val
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

fn main() {
    let mut obj = FreqStack::new();
    let options = [
        "push", "push", "push", "push", "push", "push", "debug", "pop", "pop", "pop", "pop",
    ];
    let params = [[5], [7], [5], [7], [4], [5], [0], [0], [0], [0], [0]];
    let mut result = vec![];
    for (option, param) in options.into_iter().zip(params.into_iter()) {
        match option {
            "push" => {
                obj.push(param[0]);
            }
            "pop" => {
                result.push(obj.pop());
            }
            _ => {
                println!("{:?}\n{:?}", obj.map, obj.stack);
            }
        }
    }
    println!("{:?}", result);
}
