struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let n = n as usize;
        let mut s = vec!['a'; n];
        if n % 2 == 0 {
            s[n - 1] = 'b';
        };
        s.into_iter().collect()
    }
}

fn main() {
    println!("{}", Solution::generate_the_string(10))
}
