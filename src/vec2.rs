use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn dist(&self, other: &Self) -> f64 {
        let diff = *self - *other;
        diff.norm()
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T: Into<f64>> Mul<T> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        let f: f64 = rhs.into();
        Vec2(self.0 * f, self.1 * f)
    }
}

impl<T: Into<f64>> MulAssign<T> for Vec2 {
    fn mul_assign(&mut self, rhs: T) {
        let f: f64 = rhs.into();
        self.0 *= f;
        self.1 *= f;
    }
}

impl<T: Into<f64>> Div<T> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: T) -> Self::Output {
        let f: f64 = rhs.into();
        Vec2(self.0 / f, self.1 / f)
    }
}

impl<T: Into<f64>> DivAssign<T> for Vec2 {
    fn div_assign(&mut self, rhs: T) {
        let f: f64 = rhs.into();
        self.0 /= f;
        self.1 /= f;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2(-self.0, -self.1)
    }
}
