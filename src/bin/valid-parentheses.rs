struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for c in s.chars() {
            stack.push(c);
            while stack.len() > 1 {
                match (stack[stack.len() - 2], stack[stack.len() - 1]) {
                    ('(', ')') | ('[', ']') | ('{', '}') => {
                        stack.pop();
                        stack.pop();
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
    println!("{}", Solution::is_valid("{[]}".to_string()))
}
