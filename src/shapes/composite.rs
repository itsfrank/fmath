use crate::{impl_positioned_trait, Positioned, Shape, Xform};
use std::slice::Iter;

#[derive(Clone)]
pub struct Composite {
    pub xform: Xform,
    shapes: Vec<Shape>,
}

impl Default for Composite {
    fn default() -> Self {
        Self {
            xform: Xform::default(),
            shapes: Vec::new(),
        }
    }
}

impl Composite {
    pub fn shapes_iter(&self) -> Iter<Shape> {
        self.shapes.iter()
    }
}
// impl Drawable for Composite {
//     fn draw(&self, d: &mut impl RaylibDraw) {
//         self.shapes
//             .iter()
//             .map(|s| {
//                 let mut s: Shape = s.clone();
//                 s.update_xform(self.xform);
//                 s
//             })
//             .for_each(|s| s.draw(d));
//     }
// }

impl_positioned_trait!(Composite);
