use crate::{Circle, Color, Colored, Positioned, Rect, Shape, Triangle, Vec2};

pub trait TriangleDrawer {
    fn draw_tri(&mut self, t: (Vec2, Vec2, Vec2), c: Color);
}

pub struct ShapeDrawer<D: TriangleDrawer> {
    pub d: D,
}

impl<D: TriangleDrawer> ShapeDrawer<D> {
    pub fn new(d: D) -> Self {
        Self { d }
    }

    pub fn draw_shape(&mut self, shape: Shape) {
        match shape {
            Shape::Triangle(t) => self.draw_triangle(&t),
            Shape::Rect(r) => self.draw_rect(&r),
            Shape::Circle(c) => self.draw_circle(&c),
            Shape::Composite(c) => self.draw_composite(&c),
        }
    }

    /// scale and rot are applied relative to xform.pos
    /// you probably want to make sure your points are centered around 0,0, see Triangle::centerize
    pub fn draw_triangle(&mut self, t: &Triangle) {
        if t.xform.is_zero() {
            self.d.draw_tri((t.p1, t.p2, t.p3), t.color);
        } else {
            let p1 = (t.p1.rotate_cw(t.xform.rot) * t.xform.scale) + t.xform.pos;
            let p2 = (t.p2.rotate_cw(t.xform.rot) * t.xform.scale) + t.xform.pos;
            let p3 = (t.p3.rotate_cw(t.xform.rot) * t.xform.scale) + t.xform.pos;
            self.d.draw_tri((p1, p2, p3), t.color);
        }
    }
    pub fn draw_rect(&mut self, r: &Rect) {
        let c = r.corners();
        self.d.draw_tri((c.bl, c.tr, c.tl), r.color);
        self.d.draw_tri((c.bl, c.br, c.tr), r.color);
    }

    pub fn draw_circle(&mut self, c: &Circle) {
        let r = c.radius * c.xform.scale;
        let mut ray = Vec2::new(r, 0.).rotate_cw(c.start_angle + c.xform.rot);
        let step_angle = (c.end_angle - c.start_angle) / c.sides as f32;
        for _ in 0..c.sides {
            let p0 = c.xform.pos;
            let p1 = p0 + ray;
            ray = ray.rotate_cw(step_angle);
            let p2 = p0 + ray;
            if c.end_angle < c.start_angle {
                self.d.draw_tri((p0, p1, p2), c.color);
            } else {
                self.d.draw_tri((p0, p2, p1), c.color);
            }
        }
    }

    pub fn draw_composite(&mut self, c: &crate::shapes::Composite) {
        c.shapes_iter().for_each(|s| {
            let mut s: Shape = s.clone();
            s.apply_xform(c.xform);
            if let Some(color) = c.color {
                s.set_color(color);
            }
            self.draw_shape(s);
        })
    }
}
