mod circle;
mod composite;
mod rect;
mod triangle;

pub use circle::*;
pub use composite::*;
pub use rect::*;
pub use triangle::*;

use crate::{Color, Xform};

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
#[derive(Clone)]
pub enum Shape {
    Triangle,
    Rect,
    Circle,
    Composite,
}

#[enum_dispatch(Shape)]
pub trait Positioned {
    fn get_xform(&self) -> Xform;
    fn set_xform(&mut self, new: Xform);
    fn apply_xform(&mut self, delta: Xform);
}

#[enum_dispatch(Shape)]
pub trait Colored {
    fn get_color(&self) -> Color;
    fn set_color(&mut self, new: Color);
}

#[macro_export]
macro_rules! impl_positioned_trait {
    ($struct: ident) => {
        impl Positioned for $struct {
            fn get_xform(&self) -> Xform {
                self.xform
            }
            fn set_xform(&mut self, new: Xform) {
                self.xform = new;
            }
            fn apply_xform(&mut self, delta: Xform) {
                self.xform = self.xform.apply(delta);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_colored_trait {
    ($struct: ident) => {
        impl Colored for $struct {
            fn get_color(&self) -> Color {
                self.color
            }
            fn set_color(&mut self, new: Color) {
                self.color = new;
            }
        }
    };
}
