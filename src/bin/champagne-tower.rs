struct Solution;

struct ChampagneTower {
    in_degree: Vec<Vec<f64>>,
    out_degree: Vec<Vec<f64>>,
}

impl ChampagneTower {
    pub fn new(n: usize, poured: f64) -> Self {
        let mut in_degree = vec![vec![-1.0; n]; n];
        let out_degree = vec![vec![-1.0; n]; n];
        in_degree[0][0] = poured;
        Self {
            in_degree,
            out_degree,
        }
    }

    pub fn calc_in_degree(&mut self, i: usize, j: usize) -> f64 {
        if self.in_degree[i][j].is_sign_negative() {
            let l_out_degree = if j > 0 {
                self.calc_out_degree(i - 1, j - 1)
            } else {
                0.0
            };
            let r_out_degree = if j < i {
                self.calc_out_degree(i - 1, j)
            } else {
                0.0
            };
            self.in_degree[i][j] = l_out_degree + r_out_degree;
        }
        self.in_degree[i][j]
    }

    pub fn calc_out_degree(&mut self, i: usize, j: usize) -> f64 {
        if self.out_degree[i][j].is_sign_negative() {
            let in_degree = self.calc_in_degree(i, j);
            self.out_degree[i][j] = if in_degree > 1.0 {
                (in_degree - 1.0) / 2.0
            } else {
                0.0
            }
        }
        self.out_degree[i][j]
    }
}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        ChampagneTower::new(query_row as usize + 1, poured as f64)
            .calc_in_degree(query_row as usize, query_glass as usize)
            .min(1.0)
    }
}

fn main() {
    println!("{}", Solution::champagne_tower(100000009, 33, 17));
}
