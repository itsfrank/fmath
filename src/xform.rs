use crate::{Angle, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xform {
    pub pos: Vec2,
    pub rot: Angle,
    pub scale: f32,
}

impl Default for Xform {
    fn default() -> Self {
        Self {
            pos: Vec2::zero(),
            rot: Angle::from_rad(0.),
            scale: 1.,
        }
    }
}

impl Xform {
    pub fn from_v2(v: Vec2) -> Self {
        Xform {
            pos: v,
            ..Default::default()
        }
    }

    pub fn from_xy((x, y): (f32, f32)) -> Self {
        Xform {
            pos: Vec2::new(x, y),
            ..Default::default()
        }
    }

    pub fn is_zero(&self) -> bool {
        self.pos == Vec2::zero() && self.rot == Angle::from_rad(0.) && self.scale == 1.
    }

    pub fn translate(self, v: Vec2) -> Self {
        Self {
            pos: self.pos + v,
            ..self
        }
    }

    pub fn rotate(self, a: Angle) -> Self {
        Self {
            rot: self.rot + a,
            ..self
        }
    }

    pub fn scale(self, s: f32) -> Self {
        Self {
            scale: self.scale * s,
            ..self
        }
    }

    pub fn apply(self, other: Self) -> Self {
        Self {
            pos: ((self.pos * other.scale) + other.pos).rotate_about_ccw(other.rot, other.pos),
            rot: self.rot + other.rot,
            scale: self.scale * other.scale,
        }
    }
}
