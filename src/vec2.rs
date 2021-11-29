use std::ops::{Add, AddAssign, Neg, Mul, Div, Sub, SubAssign};
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f64, 
    pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_single(a: f64) -> Self {
        Self { x: a, y: a }
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn normal(&self) -> Self {
        let hipothenuse = self.len();
        if hipothenuse > 0.0 {
            Self {
                x: self.x / hipothenuse,
                y: self.y / hipothenuse,
            }
        } else {
            self.clone()
        }
    }

    pub fn direction_to(&self, other: &Self) -> Self {
        (*other - *self).normal()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x > 0.0 { self.x / rhs.x } else { 0.0 },
            y: if rhs.y > 0.0 { self.y / rhs.y } else { 0.0 },
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn direction_0deg() {
        let v1 = Vec2::origin();
        let v2 = Vec2::new(2.0, 0.0);
        let result = Vec2::new(1.0, 0.0);

        assert_eq!(result, v1.direction_to(&v2));
    }

    #[test]
    fn direction_45deg() {
        let v1 = Vec2::origin();
        let v2 = Vec2::from_single(1.0);
        let result = Vec2::from_single(1.0/(2.0_f64).sqrt());

        assert_eq!(result, v1.direction_to(&v2));
    }

    #[test]
    fn direction_90deg() {
        let v1 = Vec2::origin();
        let v2 = Vec2::new(0.0, 23.121);
        let result = Vec2::new(0.0, 1.0);

        assert_eq!(result, v1.direction_to(&v2));
    }

    #[test]
    fn direction_180deg() {
        let v1 = Vec2::origin();
        let v2 = Vec2::new(-82.1, 0.0);
        let result = Vec2::new(-1.0, 0.0);

        assert_eq!(result, v1.direction_to(&v2));
    }

    #[test]
    fn direction_270deg() {
        let v1 = Vec2::origin();
        let v2 = Vec2::new(0.0, -2.8);
        let result = Vec2::new(0.0, -1.0);

        assert_eq!(result, v1.direction_to(&v2));
    }

    #[test]
    fn direction_not_origin() {
        let v1 = Vec2::from_single(1.0);
        let v2 = Vec2::from_single(3.0);
        let result = Vec2::from_single(1.0/(2.0_f64).sqrt());

        assert_eq!(result, v1.direction_to(&v2));
    }
}
