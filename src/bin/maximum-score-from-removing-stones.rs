struct Solution;

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut l = [a, b, c];
        l.sort();
        let [a, b, c] = l;
        if c >= a + b {
            a + b
        } else {
            (a + b + c) / 2
        }
    }
}

fn main() {
    println!("{}", Solution::maximum_score(4, 4, 6))
}
