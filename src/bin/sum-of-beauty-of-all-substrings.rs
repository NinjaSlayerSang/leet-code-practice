struct Solution;

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let n = s.len();
        if n < 3 {
            return 0;
        }
        let s = s.chars().collect::<Vec<_>>();
        let mut k = vec![[0; 26]];
        for i in 0..n {
            let mut p = k[i];
            p[s[i] as usize - 'a' as usize] += 1;
            k.push(p);
        }

        let mut r = 0;
        for i in 0..=n - 3 {
            for j in i + 3..=n {
                let a = k[i];
                let mut b = k[j];
                for t in 0..26 {
                    b[t] -= a[t];
                }
                let (min, max) = (
                    b.iter().filter(|&&x| x > 0).min().unwrap(),
                    b.iter().max().unwrap(),
                );
                r += max - min;
            }
        }
        r
    }
}

fn main() {
    println!("{}", Solution::beauty_sum("x".to_string()));
}
