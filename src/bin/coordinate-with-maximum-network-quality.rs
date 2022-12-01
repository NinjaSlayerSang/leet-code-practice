struct Solution;

impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let radius2 = radius.pow(2);
        let (lx, ly, rx, ry) = towers.iter().fold((50, 50, 0, 0), |(lx, ly, rx, ry), v| {
            let (x, y) = (v[0], v[1]);
            (lx.min(x), ly.min(y), rx.max(x), ry.max(y))
        });

        let mut r = 0;
        let (mut px, mut py) = (0, 0);

        for x in lx..=rx {
            for y in ly..=ry {
                let k = towers.iter().fold(0, |k, v| {
                    let (xi, yi, qi) = (v[0], v[1], v[2] as f64);
                    let d2 = (x - xi).pow(2) + (y - yi).pow(2);
                    k + if d2 <= radius2 {
                        (qi / (1.0 + (d2 as f64).sqrt())) as i32
                    } else {
                        0
                    }
                });
                if k > r {
                    r = k;
                    px = x;
                    py = y;
                }
            }
        }

        vec![px, py]
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::best_coordinate(
            [[0, 1, 2], [2, 1, 2], [1, 0, 2], [1, 2, 2]]
                .map(|i| i.to_vec())
                .to_vec(),
            1
        )
    );
}
