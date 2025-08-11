use crate::{impl_positioned_trait, Angle, Color, Positioned, Vec2, Xform};

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

// impl Drawable for Circle {
//     fn draw(&self, d: &mut impl RaylibDraw) {
//         d.draw_ring(
//             self.xform.pos,
//             self.inner_radius,
//             self.radius,
//             self.start_angle + self.xform.rot.as_deg(),
//             self.end_angle + self.xform.rot.as_deg(),
//             64,
//             self.color,
//         );
//     }
// }

impl_positioned_trait!(Circle);
