use fmath::{Color, ShapeDrawer, TriangleDrawer, Vec2};
use macroquad::prelude::{self as mq};
use playground::draw;

fn to_mq_v2(v: Vec2) -> mq::Vec2 {
    mq::Vec2 { x: v.x, y: v.y }
}

fn to_mq_color(c: Color) -> mq::Color {
    mq::Color::from_rgba(c.r, c.g, c.b, c.a)
}

struct MacroquadDrawer {}

impl TriangleDrawer for MacroquadDrawer {
    fn draw_tri(&mut self, t: (Vec2, Vec2, Vec2), c: Color) {
        mq::draw_triangle(to_mq_v2(t.0), to_mq_v2(t.1), to_mq_v2(t.2), to_mq_color(c));
    }
}

fn window_conf() -> macroquad::prelude::Conf {
    let (w, h) = (512, 512);
    macroquad::prelude::Conf {
        window_title: "fmath mq".to_owned(),
        window_width: w,
        window_height: h,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        draw(&mut ShapeDrawer::new(MacroquadDrawer {}));
        mq::next_frame().await
    }
}
