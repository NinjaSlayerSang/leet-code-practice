struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::iter::once;

        let mut journey = 0;
        let mut fuel = start_fuel;
        let mut queue = Vec::<i32>::with_capacity(stations.len());
        let mut result = 0;

        for station in stations.into_iter().chain(once(vec![target, 0])) {
            let distance = station[0] - journey;
            fuel -= distance;

            while fuel < 0 {
                if let Some(volume) = queue.pop() {
                    fuel += volume;
                    result += 1;
                } else {
                    return -1;
                }
            }

            journey = station[0];
            queue.push(station[1]);
            queue.sort();
        }

        result
    }
}

fn main() {
    let target = 100;
    let start_fuel = 10;
    let stations = vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]];
    println!(
        "{}",
        Solution::min_refuel_stops(target, start_fuel, stations)
    )
}
