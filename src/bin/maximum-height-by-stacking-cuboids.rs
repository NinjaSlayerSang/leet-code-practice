struct Solution;

impl Solution {
    pub fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
        let n = cuboids.len();
        cuboids.iter_mut().for_each(|v| v.sort());
        cuboids.sort();

        let mut f = vec![0; n];
        for i in 0..n {
            f[i] = cuboids[i][2];
            for j in 0..i {
                if cuboids[j][0] <= cuboids[i][0]
                    && cuboids[j][1] <= cuboids[i][1]
                    && cuboids[j][2] <= cuboids[i][2]
                {
                    f[i] = f[i].max(f[j] + cuboids[i][2]);
                }
            }
        }

        f.into_iter().max().unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_height([[38, 25, 45], [76, 35, 3]].map(|v| v.to_vec()).to_vec())
    );
}
