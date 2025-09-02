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

fn make_demo_composite() -> Composite {
    // a square with a circle on each corner and triangles pointing out
    let (w, h) = (40., 20.);
    let (hw, hh) = (w / 2., h / 2.);
    let corner_circle_base = Circle {
        radius: 2.,
        color: Color::SALMON,
        ..Default::default()
    };
    let smallest_edge = f32::min(w, h);
    let edge_tri_base = Triangle::equilateral(smallest_edge / 2., Color::LAWNGREEN);

    Composite {
        shapes: vec![
            Rect {
                width: w,
                height: h,
                color: Color::TEAL,
                ..Default::default()
            }
            .into(),
            Circle {
                xform: Xform::from_xy((hw, hh)),
                ..corner_circle_base
            }
            .into(),
            Circle {
                xform: Xform::from_xy((-hw, hh)),
                ..corner_circle_base
            }
            .into(),
            Circle {
                xform: Xform::from_xy((hw, -hh)),
                ..corner_circle_base
            }
            .into(),
            Circle {
                xform: Xform::from_xy((-hw, -hh)),
                ..corner_circle_base
            }
            .into(),
            {
                let xf = Xform {
                    pos: vec2(0., -hh),
                    ..Xform::default()
                };
                Composite {
                    shapes: vec![
                        Triangle {
                            xform: xf,
                            ..edge_tri_base
                        }
                        .into(),
                        Circle {
                            xform: xf,
                            ..corner_circle_base
                        }
                        .into(),
                    ],
                    color: None,
                    ..Default::default()
                }
            }
            .into(),
            Triangle {
                xform: Xform {
                    pos: vec2(0., hh),
                    rot: Angle::from_deg(180.),
                    ..Xform::default()
                },
                ..edge_tri_base
            }
            .into(),
            Triangle {
                xform: Xform {
                    pos: vec2(hw, 0.),
                    rot: Angle::from_deg(90.),
                    ..Xform::default()
                },
                ..edge_tri_base
            }
            .into(),
            Triangle {
                xform: Xform {
                    pos: vec2(-hw, 0.),
                    rot: Angle::from_deg(-90.),
                    ..Xform::default()
                },
                ..edge_tri_base
            }
            .into(),
        ],
        ..Default::default()
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

        let center_x = 512 / 2;
        let center_y = 512 / 2;
        let center_v2 = Vec2::new(center_x as f32, center_y as f32);

        let r = Rect {
            xform: Xform {
                pos: center_v2,
                scale: 2.,
                rot: Angle::from_deg(30.),
            },
            width: 100.,
            height: 100.,
            color: Color::GOLD,
            ..Rect::default()
        };

        let r2 = Rect {
            xform: Xform {
                pos: center_v2,
                ..Xform::default()
            },
            width: 100.,
            height: 100.,
            color: Color::RED,
            ..Rect::default()
        };
        d.draw_rect(&r);
        d.draw_rect(&r2);

        let c = Circle {
            xform: Xform {
                pos: center_v2,
                rot: Angle::from_deg(45.),
                scale: 2.,
            },
            radius: 25.,
            color: Color::LIMEGREEN,
            start_angle: Angle::zero(),
            end_angle: Angle::from_deg(90.),
            ..Circle::default()
        };
        d.draw_circle(&c);

        let iso_tri = Triangle {
            xform: Xform {
                pos: center_v2,
                rot: Angle::from_deg(45.),
                scale: 3.,
            },
            p1: Vec2::new(0., -15.),
            p2: Vec2::new(-10., 0.),
            p3: Vec2::new(10., 0.),
            color: Color::CYAN,
        }
        .centerize_points();
        d.draw_triangle(&iso_tri);

        let center_pts_tri = Triangle {
            xform: Xform {
                pos: center_v2,
                rot: Angle::from_deg(45.),
                scale: 2.,
            },
            p1: Vec2::new(0., 0.),
            p2: Vec2::new(-30., 60.),
            p3: Vec2::new(30., 60.),
            color: Color::PINK,
        };

        let center_xf_tri = Triangle {
            color: Color::PLUM,
            ..center_pts_tri
        };
        d.draw_triangle(&center_pts_tri.centerize_points());
        d.draw_triangle(&center_xf_tri);

        let center_dot = Circle {
            xform: Xform {
                pos: center_v2,
                ..Default::default()
            },
            radius: 3.,
            color: Color::WHITE,
            ..Default::default()
        };
        d.draw_circle(&center_dot);

        let mut comp = make_demo_composite();
        comp.xform = Xform {
            pos: vec2(60., 50.),
            rot: Angle::zero(),
            scale: 1.,
        };

        d.draw_composite(&comp);
        d.draw_composite(&Composite {
            xform: Xform {
                pos: vec2(120., 50.),
                rot: Angle::from_deg(45.),
                ..Default::default()
            },
            ..comp.clone()
        });
        d.draw_composite(&Composite {
            xform: Xform {
                pos: vec2(240., 50.),
                rot: Angle::from_deg(-45.),
                scale: 2.,
            },
            ..comp
        });
    }
}
