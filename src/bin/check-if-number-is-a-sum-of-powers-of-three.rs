struct Solution;

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            let m = n % 3;
            if m > 1 {
                return false;
            }
            n = n / 3;
        }
        true
    }
}

fn main() {
    println!("{}", Solution::check_powers_of_three(21));
}
