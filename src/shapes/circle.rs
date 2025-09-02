use crate::{
    impl_colored_trait, impl_positioned_trait, Angle, Color, Colored, Positioned, Vec2, Xform,
};

#[derive(Clone)]
pub struct Circle {
    pub xform: Xform,
    pub radius: f32,
    pub inner_radius: f32,
    pub start_angle: Angle,
    pub end_angle: Angle,
    pub color: Color,
    pub sides: u32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            xform: Xform::default(),
            radius: 0.,
            inner_radius: 0.,
            start_angle: Angle::zero(),
            end_angle: Angle::d_360(),
            color: Color::default(),
            sides: 32,
        }
    }
}

impl_positioned_trait!(Circle);
impl_colored_trait!(Circle);
