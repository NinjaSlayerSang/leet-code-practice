struct Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn decode(k: usize, m: usize) -> Vec<bool> {
        let mut r = vec![false; m];
        for i in 0..m {
            r[i] = k & (1 << i) > 0
        }
        r
    }

    fn encode(v: Vec<bool>) -> usize {
        v.into_iter()
            .enumerate()
            .fold(0, |r, (i, b)| r + b as usize * (1 << i))
    }

    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        let m = nums.len();
        let n = m / 2;
        nums.sort();
        let mut gcd = vec![vec![0; m]; m];
        for i in 0..m {
            for j in i..m {
                gcd[j][i] = Self::gcd(nums[j], nums[i]);
                gcd[i][j] = gcd[j][i];
            }
        }

        let l = 1 << m;
        let mut f = vec![0; l];

        for i in 0..n {
            for k in 0..l {
                let v = Self::decode(k, m);
                let c = v.iter().fold(0, |c, &b| c + !b as usize);
                if c == 2 * i {
                    for a in 0..m {
                        if v[a] == true {
                            for b in a + 1..m {
                                if v[b] == true {
                                    let mut vv = v.clone();
                                    vv[a] = false;
                                    vv[b] = false;
                                    let kk = Self::encode(vv);
                                    f[kk] = f[kk].max(f[k] + (i + 1) as i32 * gcd[a][b]);
                                }
                            }
                        }
                    }
                }
            }
        }

        f[0]
    }
}

fn main() {
    println!("{}", Solution::max_score(vec![1, 2, 3, 4, 5, 6]));
}
