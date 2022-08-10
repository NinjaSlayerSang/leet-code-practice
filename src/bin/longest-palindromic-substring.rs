struct Solution;

const MAX_LEN: usize = 1001;

type FMatrix = [[Option<bool>; MAX_LEN]; MAX_LEN];

impl Solution {
    fn u(l: usize, r: usize, ml: &mut usize, mr: &mut usize) {
        if r - l > *mr - *ml {
            *ml = l;
            *mr = r;
        }
    }

    fn f(
        l: usize,
        r: usize,
        s: &Vec<char>,
        f: &mut FMatrix,
        ml: &mut usize,
        mr: &mut usize,
    ) -> bool {
        if let Some(v) = f[l][r] {
            v
        } else {
            let check = r - l <= 1 || s[l] == s[r - 1] && Self::f(l + 1, r - 1, s, f, ml, mr);
            f[l][r] = Some(check);
            if check {
                Self::u(l, r, ml, mr)
            }
            check
        }
    }

    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut f: FMatrix = [[None; MAX_LEN]; MAX_LEN];
        let mut ml = 0;
        let mut mr = 0;

        for l in 0..=len {
            for r in l..=len {
                Self::f(l, r, &s, &mut f, &mut ml, &mut mr);
            }
        }

        s[ml..mr].into_iter().collect()
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("bananas".to_string()))
}
