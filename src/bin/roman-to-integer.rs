struct Solution;

const MAPS: [(char, i32); 7] = [
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];

impl Solution {
    fn get_num(k: char) -> i32 {
        for (c, n) in MAPS {
            if c == k {
                return n;
            }
        }
        panic!()
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut s = s.chars().map(Self::get_num).collect::<Vec<_>>();
        s.push(0);
        let mut result = 0;
        for i in 0..s.len() - 1 {
            if s[i] < s[i + 1] {
                result -= s[i]
            } else {
                result += s[i]
            }
        }
        result
    }
}

fn main() {
    println!("{}", Solution::roman_to_int("MCMXCIV".to_string()))
}
