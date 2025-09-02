use crate::{impl_positioned_trait, Color, Colored, Positioned, Shape, Xform};
use std::slice::Iter;

#[derive(Clone, Default)]
pub struct Composite {
    pub xform: Xform,
    pub color: Option<Color>,
    pub shapes: Vec<Shape>,
}

impl Composite {
    pub fn shapes_iter(&self) -> Iter<Shape> {
        self.shapes.iter()
    }
}

impl_positioned_trait!(Composite);

impl Colored for Composite {
    fn get_color(&self) -> Color {
        self.color.unwrap_or_default() // could we do better here?
    }

    fn set_color(&mut self, new: Color) {
        self.color = Some(new)
    }
}
