use crate::{Color, Vec2, Xform};

use super::Positioned;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Triangle {
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
    pub color: Color,
}

impl Triangle {
    pub fn new(p1: Vec2, p2: Vec2, p3: Vec2, color: Color) -> Self {
        Self { p1, p2, p3, color }
    }

    pub fn from_pts(pts: (Vec2, Vec2, Vec2), color: Color) -> Self {
        Self {
            p1: pts.0,
            p2: pts.1,
            p3: pts.2,
            color,
        }
    }
}

impl Positioned for Triangle {
    fn get_xform(&self) -> Xform {
        todo!()
    }
    fn set_xform(&mut self, new: Xform) {
        todo!()
    }
    fn apply_xform(&mut self, delta: Xform) {
        todo!()
    }
}
