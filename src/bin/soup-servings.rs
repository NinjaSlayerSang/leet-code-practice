struct Solution;

impl Solution {
    fn f(mem: &mut Vec<Vec<f64>>, i: i32, j: i32) -> f64 {
        let (ii, jj) = (i as usize, j as usize);
        if mem[ii][jj].is_sign_negative() {
            mem[ii][jj] = 0.25
                * (Self::f(mem, 0.max(i - 4), j)
                    + Self::f(mem, 0.max(i - 3), 0.max(j - 1))
                    + Self::f(mem, 0.max(i - 2), 0.max(j - 2))
                    + Self::f(mem, 0.max(i - 1), 0.max(j - 3)));
        }
        mem[ii][jj]
    }

    pub fn soup_servings(n: i32) -> f64 {
        let n = n as usize / 25 + (n % 25 > 0) as usize;

        if n > 180 {
            1.0
        } else {
            let mut mem = vec![vec![-1.0; n + 1]; n + 1];
            for i in 1..=n {
                mem[i][0] = 0.0;
                mem[0][i] = 1.0;
            }
            mem[0][0] = 0.5;

            Self::f(&mut mem, n as i32, n as i32)
        }
    }
}

fn main() {
    println!("{}", Solution::soup_servings(100));
}
