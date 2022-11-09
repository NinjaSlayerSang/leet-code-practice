struct Solution;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let nn = n as usize;
        let mut result = vec![vec![0; nn]; nn];

        for i in 0..n {
            for j in 0..n {
                let (ii, jj) = (i as usize, j as usize);
                result[ii][jj] = i32::min(i32::min(i + 1, n - i), i32::min(j + 1, n - j));
            }
        }

        for mine in mines.iter() {
            let (x, y) = (mine[0] as usize, mine[1] as usize);
            result[x][y] = 0;
        }

        for mine in mines {
            let (x, y) = (mine[0], mine[1]);
            let (xx, yy) = (x as usize, y as usize);

            for k in 1..n - x {
                let kk = k as usize;
                if result[xx + kk][yy] > 0 {
                    if result[xx + kk][yy] > k {
                        result[xx + kk][yy] = k;
                    }
                } else {
                    break;
                }
            }

            for k in 1..=x {
                let kk = k as usize;
                if result[xx - kk][yy] > 0 {
                    if result[xx - kk][yy] > k {
                        result[xx - kk][yy] = k;
                    }
                } else {
                    break;
                }
            }

            for k in 1..n - y {
                let kk = k as usize;
                if result[xx][yy + kk] > 0 {
                    if result[xx][yy + kk] > k {
                        result[xx][yy + kk] = k;
                    }
                } else {
                    break;
                }
            }

            for k in 1..=y {
                let kk = k as usize;
                if result[xx][yy - kk] > 0 {
                    if result[xx][yy - kk] > k {
                        result[xx][yy - kk] = k;
                    }
                } else {
                    break;
                }
            }
        }

        result
            .into_iter()
            .flat_map(|r| r.into_iter())
            .max()
            .unwrap_or(0)
    }
}

fn main() {
    println!(
        "{}",
        Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]])
    );
}
