use crate::Angle;
use std::ops::{Add, Div, Mul, Sub};

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_xy((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0., 0.)
    }

    pub fn one() -> Self {
        Self::new(1., 1.)
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn angle_to(self, rhs: Self) -> Angle {
        Angle::from_rad((self.dot(rhs) / (self.len() * rhs.len())).acos())
    }

    /// Rotates vector counter-clockwise about `t` with origin `c`
    pub fn rotate_about_ccw(self, t: Angle, c: Self) -> Self {
        let (sin, cos) = t.as_rad().sin_cos();
        Self {
            x: ((self.x - c.x) * cos - (self.y - c.y) * sin) + c.x,
            y: ((self.x - c.x) * sin + (self.y - c.y) * cos) + c.y,
        }
    }

    /// Rotates vector clockwise about `t` with origin `c`
    pub fn rotate_about_cw(self, t: Angle, c: Self) -> Self {
        self.rotate_about_ccw(t * -1., c)
    }

    /// Rotates vector counter-clockwise by `t`
    pub fn rotate_ccw(self, t: Angle) -> Self {
        let (sin, cos) = t.as_rad().sin_cos();
        let sin = -sin; // correction for a y-is-down system (e.g. raylib)
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Rotates vector clockwise by `t`
    pub fn rotate_cw(self, t: Angle) -> Self {
        self.rotate_ccw(t * -1.)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(v: (f32, f32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl From<Vec2> for (f32, f32) {
    fn from(val: Vec2) -> Self {
        (val.x, val.y)
    }
}

pub fn triangulate_rect(pos: Vec2, dim: Vec2) -> Vec<(Vec2, Vec2, Vec2)> {
    let (hw, hh) = (dim / 2.).into();

    let tl = pos + vec2(-hw, -hh);
    let tr = pos + vec2(hw, -hh);
    let br = pos + vec2(hw, hh);
    let bl = pos + vec2(-hw, hh);

    vec![(tr, tl, bl), (tr, bl, br)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn add_and_mul() {
        let v1 = Vec2::from((1., 2.));
        let (x1, y1) = v1.into();
        assert_eq!(x1, 1.);
        assert_eq!(y1, 2.);

        let v2 = v1 + Vec2::new(3., 4.);
        assert_eq!(v2, Vec2::from((4., 6.)));

        let v3 = v2 * 2.;
        assert_eq!(v3, Vec2::from((8., 12.)));
    }

    #[test]
    fn rotations() {
        let v1 = Vec2::from((3., 2.));
        let v2 = v1.rotate_cw(Angle::from_deg(90.));
        assert_relative_eq!(v2.x, 2.);
        assert_relative_eq!(v2.y, -3.);

        let v3 = v1.rotate_ccw(Angle::from_deg(90.));
        assert_relative_eq!(v3.x, -2.);
        assert_relative_eq!(v3.y, 3.);

        let about = Vec2::from((1., 1.));
        let v4 = v1.rotate_about_cw(Angle::from_deg(90.), about);
        assert_relative_eq!(v4.x, 2.);
        assert_relative_eq!(v4.y, -1.);

        let v5 = v1.rotate_about_ccw(Angle::from_deg(90.), about);
        assert_relative_eq!(v5.x, 0.);
        assert_relative_eq!(v5.y, 3.);
    }

    #[test]
    fn angle_to() {
        let e = vec2(1., 0.);
        let se = vec2(1., -1.);
        let s = vec2(0., -1.);
        let sw = vec2(-1., -1.);
        let w = vec2(-1., 0.);
        let nw = vec2(-1., 1.);
        let n = vec2(0., 1.);
        let ne = vec2(1., 1.);

        assert_relative_eq!(e.angle_to(e).as_deg(), 0.);
        assert_relative_eq!(e.angle_to(se).as_deg(), 45.);
        assert_relative_eq!(e.angle_to(s).as_deg(), 90.);
        assert_relative_eq!(e.angle_to(sw).as_deg(), 135.);
        assert_relative_eq!(e.angle_to(w).as_deg(), 180.);
        assert_relative_eq!(e.angle_to(nw).as_deg(), 135.);
        assert_relative_eq!(e.angle_to(n).as_deg(), 90.);
        assert_relative_eq!(e.angle_to(ne).as_deg(), 45.);
    }
}
