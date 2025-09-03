use playground::draw;
use fmath::{Color, ShapeDrawer, TriangleDrawer, Vec2};
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
    while !rl_handle.window_should_close() {
        let mut d = ShapeDrawer::new(RaylibDrawer {
            rl_d: rl_handle.begin_drawing(&thread),
        });

        d.d.rl_d.clear_background(rl::Color::BLANK);

        draw(&mut d);
    }
}
