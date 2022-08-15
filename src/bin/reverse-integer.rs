struct Solution;

impl Solution {
    fn f(x: i32) -> Option<i32> {
        let s = if x.is_negative() { -1 } else { 1 };
        let mut r = x.checked_abs()?;
        let mut n = Vec::<i32>::new();
        while r > 0 {
            let k = r % 10;
            n.push(k);
            r = r / 10;
        }
        n.into_iter()
            .rev()
            .enumerate()
            .fold(Some(0i32), |r, (i, k)| {
                r?.checked_add(10i32.checked_pow(i as u32)?.checked_mul(k)?)
            })?
            .checked_mul(s)
    }

    pub fn reverse(x: i32) -> i32 {
        Self::f(x).unwrap_or(0)
    }
}

fn main() {
    println!("{}", Solution::reverse(-12345))
}
