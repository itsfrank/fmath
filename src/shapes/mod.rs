mod circle;
mod composite;
mod rect;
mod triangle;

pub use circle::*;
pub use composite::*;
pub use rect::*;
pub use triangle::*;

use crate::Xform;

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
