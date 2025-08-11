use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Angle {
    rad: f32,
}

impl Default for Angle {
    fn default() -> Self {
        Self::from_rad(0.)
    }
}

impl Angle {
    pub const fn zero() -> Self {
        Self { rad: 0. }
    }
    pub const fn r_pi() -> Self {
        Self {
            rad: std::f32::consts::PI,
        }
    }
    pub const fn r_2pi() -> Self {
        Self {
            rad: std::f32::consts::PI * 2.,
        }
    }
    pub const fn d_180() -> Self {
        Self::r_pi()
    }
    pub const fn d_360() -> Self {
        Self::r_2pi()
    }

    pub fn from_rad(rad: f32) -> Self {
        Self { rad }
    }
    pub fn from_deg(deg: f32) -> Self {
        Self {
            rad: deg.to_radians(),
        }
    }
    pub fn as_rad(&self) -> f32 {
        self.rad
    }
    pub fn as_deg(&self) -> f32 {
        self.rad.to_degrees()
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Angle {
            rad: self.rad + rhs.rad,
        }
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Angle {
            rad: self.rad - rhs.rad,
        }
    }
}
impl Mul<f32> for Angle {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            rad: self.rad * rhs,
        }
    }
}
impl Div<f32> for Angle {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            rad: self.rad / rhs,
        }
    }
}
