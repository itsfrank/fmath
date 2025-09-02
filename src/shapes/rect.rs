use crate::{impl_colored_trait, impl_positioned_trait, Color, Colored, Positioned, Vec2, Xform};

#[derive(Debug, Clone)]
pub struct Rect {
    pub xform: Xform,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub corner_radius: f32,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            xform: Xform::default(),
            width: 0.,
            height: 0.,
            color: Color::default(),
            corner_radius: 0.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RectCorners {
    pub tl: Vec2,
    pub tr: Vec2,
    pub br: Vec2,
    pub bl: Vec2,
}

pub struct RectCornersIter {
    corners: RectCorners,
    index: usize,
}

impl Iterator for RectCornersIter {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.index {
            0 => Some(self.corners.tl),
            1 => Some(self.corners.tr),
            2 => Some(self.corners.br),
            3 => Some(self.corners.bl),
            _ => None,
        };
        self.index += 1;
        item
    }
}

impl IntoIterator for RectCorners {
    type Item = Vec2;
    type IntoIter = RectCornersIter;
    fn into_iter(self) -> Self::IntoIter {
        RectCornersIter {
            corners: self,
            index: 0,
        }
    }
}

impl Rect {
    pub fn corners(&self) -> RectCorners {
        let w = self.width * self.xform.scale;
        let h = self.height * self.xform.scale;
        let wh = Vec2::new(w / 2., h / 2.);
        let wmh = Vec2::new(w / 2., -h / 2.);

        let mut corners = RectCorners {
            tr: wmh,
            bl: wmh * -1.,
            tl: wh * -1.,
            br: wh,
        };

        // apply rotation
        if self.xform.rot != crate::Angle::zero() {
            corners.tl = corners.tl.rotate_cw(self.xform.rot);
            corners.tr = corners.tr.rotate_cw(self.xform.rot);
            corners.br = corners.br.rotate_cw(self.xform.rot);
            corners.bl = corners.bl.rotate_cw(self.xform.rot);
        }

        // apply pos
        corners.tl = corners.tl + self.xform.pos;
        corners.tr = corners.tr + self.xform.pos;
        corners.br = corners.br + self.xform.pos;
        corners.bl = corners.bl + self.xform.pos;

        corners
    }
}

// impl Drawable for Rect {
//     fn draw(&self, d: &mut impl RaylibDraw) {
//         if self.corner_radius == 0. {
//             let rlrec = Rectangle::new(self.xform.pos.x, self.xform.pos.y, self.width, self.height);
//             let origin = Vec2::new(self.width / 2., self.height / 2.);
//             d.draw_rectangle_pro(rlrec, origin, self.xform.rot.as_deg(), self.color);
//         } else {
//             let corner_radius = self.corner_radius.min(self.height.min(self.width) / 2.); // clamp
//             let fill_width = self.width - (2. * corner_radius);
//             let fill_height = self.height - (2. * corner_radius);
//
//             let rl_rec1 =
//                 Rectangle::new(self.xform.pos.x, self.xform.pos.y, fill_width, self.height);
//             let origin1 = Vec2::new(fill_width / 2., self.height / 2.);
//             let rl_rec2 =
//                 Rectangle::new(self.xform.pos.x, self.xform.pos.y, self.width, fill_height);
//             let origin2 = Vec2::new(self.width / 2., fill_height / 2.);
//
//             d.draw_rectangle_pro(rl_rec1, origin1, self.xform.rot.as_deg(), self.color);
//             d.draw_rectangle_pro(rl_rec2, origin2, self.xform.rot.as_deg(), self.color);
//
//             let corners = self.corners();
//             let rr = Vec2::new(corner_radius, corner_radius);
//             let rmr = Vec2::new(corner_radius, -corner_radius);
//             d.draw_circle_sector(corners.tl + rr, corner_radius, 180., 270., 32, self.color);
//             d.draw_circle_sector(corners.tr - rmr, corner_radius, 270., 360., 32, self.color);
//             d.draw_circle_sector(corners.br - rr, corner_radius, 0., 90., 32, self.color);
//             d.draw_circle_sector(corners.bl + rmr, corner_radius, 90., 180., 32, self.color);
//         }
//     }
// }

impl_positioned_trait!(Rect);
impl_colored_trait!(Rect);

#[cfg(test)]
mod test {
    use super::*;
    use crate::shapes::Xform;

    // use approx::assert_relative_eq;

    macro_rules! assert_v2_eq {
        ( $l:expr, $r:expr ) => {
            approx::assert_relative_eq!($l.x, $r.x);
            approx::assert_relative_eq!($l.y, $r.y);
        };
    }

    #[test]
    fn test_corners() {
        let corners = Rect {
            xform: Xform::from_xy((10., 15.)),
            width: 30.,
            height: 24.,
            ..Default::default()
        }
        .corners();

        assert_v2_eq!(corners.tl, Vec2::new(10. - (30. / 2.), 15. - (24. / 2.)));
        assert_v2_eq!(corners.tr, Vec2::new(10. + (30. / 2.), 15. - (24. / 2.)));
        assert_v2_eq!(corners.br, Vec2::new(10. + (30. / 2.), 15. + (24. / 2.)));
        assert_v2_eq!(corners.bl, Vec2::new(10. - (30. / 2.), 15. + (24. / 2.)));
    }
}
