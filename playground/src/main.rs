use fmath::*;
use raylib::prelude::{self as rl, RaylibDraw};

fn to_rl_v2(v: Vec2) -> rl::Vector2 {
    rl::Vector2 { x: v.x, y: v.y }
}

fn to_rl_color(c: Color) -> rl::Color {
    rl::Color::new(c.r, c.g, c.b, c.a)
}

struct RaylibDrawer<'a> {
    pub rl_d: rl::RaylibDrawHandle<'a>,
}

impl TriangleDrawer for RaylibDrawer<'_> {
    fn draw_tri(&mut self, t: (Vec2, Vec2, Vec2), c: Color) {
        self.rl_d
            .draw_triangle(to_rl_v2(t.0), to_rl_v2(t.1), to_rl_v2(t.2), to_rl_color(c));
    }
}

fn main() {
    // Create a Raylib handle and thread
    let (mut rl_handle, thread) = raylib::init().size(512, 512).title("Red Circle").build();

    // Main game loop
    let mut start_angle = Angle::from_rad(0.);
    let mut end_angle = Angle::from_rad(std::f32::consts::PI);
    while !rl_handle.window_should_close() {
        let mut d = ShapeDrawer::new(RaylibDrawer {
            rl_d: rl_handle.begin_drawing(&thread),
        });

        d.d.rl_d.clear_background(rl::Color::BLANK);

        // Draw a red circle in the center
        let center_x = 512 / 2;
        let center_y = 512 / 2;

        let r = Rect {
            xform: Xform::from_xy((center_x as f32, center_y as f32)),
            width: 100.,
            height: 100.,
            color: Color::GOLDENROD,
            ..Rect::default()
        };
        d.draw_rect(&r);

        let c = Circle {
            xform: Xform::from_xy((center_x as f32, center_y as f32)),
            radius: 45.,
            color: Color::GREEN,
            start_angle,
            end_angle,
            ..Circle::default()
        };
        d.draw_circle(&c);
        start_angle = start_angle + Angle::from_deg(0.2);
        end_angle = end_angle + Angle::from_deg(0.1);
    }
}
