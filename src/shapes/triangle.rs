use std::f32::{self, consts};

use crate::{impl_colored_trait, impl_positioned_trait, vec2, Color, Colored, Vec2, Xform};

use super::Positioned;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Triangle {
    pub xform: Xform,
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
    pub color: Color,
}

impl Triangle {
    pub fn new(p1: Vec2, p2: Vec2, p3: Vec2, color: Color) -> Self {
        Self::from_pts((p1, p2, p3), color)
    }

    pub fn from_pts(pts: (Vec2, Vec2, Vec2), color: Color) -> Self {
        Self {
            xform: Xform::default(),
            p1: pts.0,
            p2: pts.1,
            p3: pts.2,
            color,
        }
    }

    pub fn equilateral(side: f32, color: Color) -> Self {
        let sqrt_3_2: f32 = f32::sqrt(3.) / 2.;
        let height = side * sqrt_3_2;
        let hside = side / 2.;
        Triangle {
            p1: vec2(0., -height),
            p2: vec2(-hside, 0.),
            p3: vec2(hside, 0.),
            color,
            ..Default::default()
        }
    }

    /// adjusts the points to make the center of the triangle 0,0
    /// xform is untouched, triangle will move to its center
    pub fn centerize_points(&self) -> Self {
        let center = (self.p1 + self.p2 + self.p3) / 3.;
        Triangle {
            p1: self.p1 - center,
            p2: self.p2 - center,
            p3: self.p3 - center,
            ..*self
        }
    }

    /// adjusts point to make the center of the triangle 0,0
    /// both points and xform will be changed, triangle will remain in exact location
    /// useful to create a triangle in a location relative to a point and centerize after
    pub fn centerize_xform(&self) -> Triangle {
        let center = (self.p1 + self.p2 + self.p3) / 3.;
        let centerized = self.centerize_points();
        Triangle {
            xform: Xform {
                pos: self.xform.pos + center,
                ..centerized.xform
            },
            ..centerized
        }
    }
}

impl_positioned_trait!(Triangle);
impl_colored_trait!(Triangle);
