use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Sub};

struct Solution;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec2(f64, f64);

const VEC2_ZERO: Vec2 = Vec2(0.0, 0.0);

impl From<Vec<i32>> for Vec2 {
    fn from(v: Vec<i32>) -> Self {
        Self(v[0] as f64, v[1] as f64)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Vec2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Vec2 {
    fn len(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }
}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut p: [Vec2; 4] = [p1.into(), p2.into(), p3.into(), p4.into()];
        let mut center = VEC2_ZERO;
        for v in p {
            center += v;
        }
        center /= 4.0;
        for v in p.iter_mut() {
            *v = *v - center;
        }

        let mut backward = Vec::<Vec2>::with_capacity(3);
        let mut vertical = Vec::<Vec2>::with_capacity(3);
        let main = p[0];
        for i in 1..=3 {
            let target = p[i];
            if main + target == VEC2_ZERO {
                backward.push(target);
            } else if main * target == 0.0 {
                vertical.push(target);
            }
        }

        backward.len() == 1
            && vertical.len() == 2
            && vertical[0] + vertical[1] == VEC2_ZERO
            && main.len() == vertical[0].len()
    }
}

fn main() {
    println!(
        "{}",
        Solution::valid_square(vec![1, 0], vec![0, 1], vec![1, 1], vec![0, 0])
    )
}
