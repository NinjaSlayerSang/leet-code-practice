struct Solution;

const M: i64 = 1000000007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![[0i64; 2]; n + 1];

        f[0] = [1, 0];
        f[1] = [1, 1];

        for i in 2..=n {
            f[i] = [
                (f[i - 1][0] + f[i - 2][0] + 2 * f[i - 2][1]) % M,
                (f[i - 1][0] + f[i - 1][1]) % M,
            ];
        }

        f[n][0] as i32
    }
}

fn main() {
    println!("{}", Solution::num_tilings(5))
}
