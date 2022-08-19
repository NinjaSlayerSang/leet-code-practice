struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        (0..start_time.len()).fold(0, |r, i| {
            r + (start_time[i] <= query_time && query_time <= end_time[i]) as i32
        })
    }
}

fn main() {
    println!(
        "{}",
        Solution::busy_student(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
            5
        )
    )
}
