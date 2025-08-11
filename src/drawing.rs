use crate::{Circle, Color, Positioned, Rect, Shape, Triangle, Vec2};

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
            Shape::Triangle(t) => self.draw_triangle(t),
            Shape::Rect(r) => self.draw_rect(&r),
            Shape::Circle(c) => self.draw_circle(&c),
            Shape::Composite(c) => self.draw_composite(&c),
        }
    }

    pub fn draw_triangle(&mut self, t: Triangle) {
        self.d.draw_tri((t.p1, t.p2, t.p3), t.color);
    }
    pub fn draw_rect(&mut self, r: &Rect) {
        let c = r.corners();
        self.d.draw_tri((c.bl, c.tr, c.tl), r.color);
        self.d.draw_tri((c.bl, c.br, c.tr), r.color);
    }

    pub fn draw_circle(&mut self, r: &Circle) {
        let mut ray = Vec2::new(r.radius, 0.).rotate_cw(r.start_angle);
        let step_angle = (r.end_angle - r.start_angle) / r.sides as f32;
        for _ in 0..r.sides {
            let p0 = r.xform.pos;
            let p1 = p0 + ray;
            ray = ray.rotate_cw(step_angle);
            let p2 = p0 + ray;
            if r.end_angle < r.start_angle {
                self.d.draw_tri((p0, p1, p2), r.color);
            } else {
                self.d.draw_tri((p0, p2, p1), r.color);
            }
        }
    }

    pub fn draw_composite(&mut self, c: &crate::shapes::Composite) {
        c.shapes_iter().for_each(|s| {
            let mut s: Shape = s.clone();
            s.apply_xform(c.xform);
            self.draw_shape(s);
        })
    }
}
