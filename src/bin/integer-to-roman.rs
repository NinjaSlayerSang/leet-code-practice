struct Solution;

const MAPS: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Solution {
    fn cut(num: i32) -> (usize, i32) {
        for i in 0..13 {
            let r = num - MAPS[i].0;
            if r >= 0 {
                return (i, r);
            }
        }
        panic!()
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut remain = num;
        let mut result = String::new();
        while remain > 0 {
            let (i, r) = Self::cut(remain);
            result += MAPS[i].1;
            remain = r;
        }
        result
    }
}

pub fn main() {
    println!("{}", Solution::int_to_roman(1994))
}
