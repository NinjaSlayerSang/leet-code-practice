struct Solution;

const M: i64 = 1000000007;

impl Solution {
    fn gcd(a: i64, b: i64) -> i64 {
        let c = a % b;
        if c == 0 {
            b
        } else {
            Self::gcd(b, c)
        }
    }

    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        use std::iter::once;

        let n = n as i64;
        let (a, b) = (a.max(b) as i64, a.min(b) as i64);
        let g = Self::gcd(a, b);
        let i = b / g;
        let j = a / g;
        let s = i * j * g;

        let mut l = once(0)
            .chain((1..i).into_iter().map(|e| e * a))
            .chain((1..j).into_iter().map(|e| e * b))
            .collect::<Vec<_>>();
        l.sort();

        let t = i + j - 1;
        let k = n / t;
        let r = n % t;

        ((k * s + l[r as usize]) % M) as i32
    }
}

fn main() {
    println!("{}", Solution::nth_magical_number(4, 2, 3));
}
