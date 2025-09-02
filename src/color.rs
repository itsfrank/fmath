#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }
    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

// taken from raylib-rs core/color.rs with a small degree of shame
impl Color {
    pub const INDIANRED: Color = Color::from_rgb(205, 92, 92);
    pub const LIGHTCORAL: Color = Color::from_rgb(240, 128, 128);
    pub const SALMON: Color = Color::from_rgb(250, 128, 114);
    pub const DARKSALMON: Color = Color::from_rgb(233, 150, 122);
    pub const LIGHTSALMON: Color = Color::from_rgb(255, 160, 122);
    pub const CRIMSON: Color = Color::from_rgb(220, 20, 60);
    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const FIREBRICK: Color = Color::from_rgb(178, 34, 34);
    pub const DARKRED: Color = Color::from_rgb(139, 0, 0);
    pub const PINK: Color = Color::from_rgb(255, 192, 203);
    pub const LIGHTPINK: Color = Color::from_rgb(255, 182, 193);
    pub const HOTPINK: Color = Color::from_rgb(255, 105, 180);
    pub const DEEPPINK: Color = Color::from_rgb(255, 20, 147);
    pub const MEDIUMVIOLETRED: Color = Color::from_rgb(199, 21, 133);
    pub const PALEVIOLETRED: Color = Color::from_rgb(219, 112, 147);
    pub const CORAL: Color = Color::from_rgb(255, 127, 80);
    pub const TOMATO: Color = Color::from_rgb(255, 99, 71);
    pub const ORANGERED: Color = Color::from_rgb(255, 69, 0);
    pub const DARKORANGE: Color = Color::from_rgb(255, 140, 0);
    pub const ORANGE: Color = Color::from_rgb(255, 165, 0);
    pub const GOLD: Color = Color::from_rgb(255, 215, 0);
    pub const YELLOW: Color = Color::from_rgb(255, 255, 0);
    pub const LIGHTYELLOW: Color = Color::from_rgb(255, 255, 224);
    pub const LEMONCHIFFON: Color = Color::from_rgb(255, 250, 205);
    pub const LIGHTGOLDENRODYELLOW: Color = Color::from_rgb(250, 250, 210);
    pub const PAPAYAWHIP: Color = Color::from_rgb(255, 239, 213);
    pub const MOCCASIN: Color = Color::from_rgb(255, 228, 181);
    pub const PEACHPUFF: Color = Color::from_rgb(255, 218, 185);
    pub const PALEGOLDENROD: Color = Color::from_rgb(238, 232, 170);
    pub const KHAKI: Color = Color::from_rgb(240, 230, 140);
    pub const DARKKHAKI: Color = Color::from_rgb(189, 183, 107);
    pub const LAVENDER: Color = Color::from_rgb(230, 230, 250);
    pub const THISTLE: Color = Color::from_rgb(216, 191, 216);
    pub const PLUM: Color = Color::from_rgb(221, 160, 221);
    pub const VIOLET: Color = Color::from_rgb(238, 130, 238);
    pub const ORCHID: Color = Color::from_rgb(218, 112, 214);
    pub const FUCHSIA: Color = Color::from_rgb(255, 0, 255);
    pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);
    pub const MEDIUMORCHID: Color = Color::from_rgb(186, 85, 211);
    pub const MEDIUMPURPLE: Color = Color::from_rgb(147, 112, 219);
    pub const REBECCAPURPLE: Color = Color::from_rgb(102, 51, 153);
    pub const BLUEVIOLET: Color = Color::from_rgb(138, 43, 226);
    pub const DARKVIOLET: Color = Color::from_rgb(148, 0, 211);
    pub const DARKORCHID: Color = Color::from_rgb(153, 50, 204);
    pub const DARKMAGENTA: Color = Color::from_rgb(139, 0, 139);
    pub const PURPLE: Color = Color::from_rgb(128, 0, 128);
    pub const DARKPURPLE: Color = Color::from_rgb(112, 31, 126);
    pub const INDIGO: Color = Color::from_rgb(75, 0, 130);
    pub const SLATEBLUE: Color = Color::from_rgb(106, 90, 205);
    pub const DARKSLATEBLUE: Color = Color::from_rgb(72, 61, 139);
    pub const MEDIUMSLATEBLUE: Color = Color::from_rgb(123, 104, 238);
    pub const GREENYELLOW: Color = Color::from_rgb(173, 255, 47);
    pub const CHARTREUSE: Color = Color::from_rgb(127, 255, 0);
    pub const LAWNGREEN: Color = Color::from_rgb(124, 252, 0);
    pub const LIME: Color = Color::from_rgb(0, 255, 0);
    pub const LIMEGREEN: Color = Color::from_rgb(50, 205, 50);
    pub const PALEGREEN: Color = Color::from_rgb(152, 251, 152);
    pub const LIGHTGREEN: Color = Color::from_rgb(144, 238, 144);
    pub const MEDIUMSPRINGGREEN: Color = Color::from_rgb(0, 250, 154);
    pub const SPRINGGREEN: Color = Color::from_rgb(0, 255, 127);
    pub const MEDIUMSEAGREEN: Color = Color::from_rgb(60, 179, 113);
    pub const SEAGREEN: Color = Color::from_rgb(46, 139, 87);
    pub const FORESTGREEN: Color = Color::from_rgb(34, 139, 34);
    pub const GREEN: Color = Color::from_rgb(0, 128, 0);
    pub const DARKGREEN: Color = Color::from_rgb(0, 100, 0);
    pub const YELLOWGREEN: Color = Color::from_rgb(154, 205, 50);
    pub const OLIVEDRAB: Color = Color::from_rgb(107, 142, 35);
    pub const OLIVE: Color = Color::from_rgb(128, 128, 0);
    pub const DARKOLIVEGREEN: Color = Color::from_rgb(85, 107, 47);
    pub const MEDIUMAQUAMARINE: Color = Color::from_rgb(102, 205, 170);
    pub const DARKSEAGREEN: Color = Color::from_rgb(143, 188, 139);
    pub const LIGHTSEAGREEN: Color = Color::from_rgb(32, 178, 170);
    pub const DARKCYAN: Color = Color::from_rgb(0, 139, 139);
    pub const TEAL: Color = Color::from_rgb(0, 128, 128);
    pub const AQUA: Color = Color::from_rgb(0, 255, 255);
    pub const CYAN: Color = Color::from_rgb(0, 255, 255);
    pub const LIGHTCYAN: Color = Color::from_rgb(224, 255, 255);
    pub const PALETURQUOISE: Color = Color::from_rgb(175, 238, 238);
    pub const AQUAMARINE: Color = Color::from_rgb(127, 255, 212);
    pub const TURQUOISE: Color = Color::from_rgb(64, 224, 208);
    pub const MEDIUMTURQUOISE: Color = Color::from_rgb(72, 209, 204);
    pub const DARKTURQUOISE: Color = Color::from_rgb(0, 206, 209);
    pub const CADETBLUE: Color = Color::from_rgb(95, 158, 160);
    pub const STEELBLUE: Color = Color::from_rgb(70, 130, 180);
    pub const LIGHTSTEELBLUE: Color = Color::from_rgb(176, 196, 222);
    pub const POWDERBLUE: Color = Color::from_rgb(176, 224, 230);
    pub const LIGHTBLUE: Color = Color::from_rgb(173, 216, 230);
    pub const SKYBLUE: Color = Color::from_rgb(135, 206, 235);
    pub const LIGHTSKYBLUE: Color = Color::from_rgb(135, 206, 250);
    pub const DEEPSKYBLUE: Color = Color::from_rgb(0, 191, 255);
    pub const DODGERBLUE: Color = Color::from_rgb(30, 144, 255);
    pub const CORNFLOWERBLUE: Color = Color::from_rgb(100, 149, 237);
    pub const ROYALBLUE: Color = Color::from_rgb(65, 105, 225);
    pub const BLUE: Color = Color::from_rgb(0, 0, 255);
    pub const MEDIUMBLUE: Color = Color::from_rgb(0, 0, 205);
    pub const DARKBLUE: Color = Color::from_rgb(0, 0, 139);
    pub const NAVY: Color = Color::from_rgb(0, 0, 128);
    pub const MIDNIGHTBLUE: Color = Color::from_rgb(25, 25, 112);
    pub const CORNSILK: Color = Color::from_rgb(255, 248, 220);
    pub const BLANCHEDALMOND: Color = Color::from_rgb(255, 235, 205);
    pub const BISQUE: Color = Color::from_rgb(255, 228, 196);
    pub const NAVAJOWHITE: Color = Color::from_rgb(255, 222, 173);
    pub const WHEAT: Color = Color::from_rgb(245, 222, 179);
    pub const BURLYWOOD: Color = Color::from_rgb(222, 184, 135);
    pub const TAN: Color = Color::from_rgb(210, 180, 140);
    pub const ROSYBROWN: Color = Color::from_rgb(188, 143, 143);
    pub const SANDYBROWN: Color = Color::from_rgb(244, 164, 96);
    pub const GOLDENROD: Color = Color::from_rgb(218, 165, 32);
    pub const DARKGOLDENROD: Color = Color::from_rgb(184, 134, 11);
    pub const PERU: Color = Color::from_rgb(205, 133, 63);
    pub const CHOCOLATE: Color = Color::from_rgb(210, 105, 30);
    pub const SADDLEBROWN: Color = Color::from_rgb(139, 69, 19);
    pub const SIENNA: Color = Color::from_rgb(160, 82, 45);
    pub const BROWN: Color = Color::from_rgb(165, 42, 42);
    pub const DARKBROWN: Color = Color::from_rgb(76, 63, 47);
    pub const MAROON: Color = Color::from_rgb(128, 0, 0);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const SNOW: Color = Color::from_rgb(255, 250, 250);
    pub const HONEYDEW: Color = Color::from_rgb(240, 255, 240);
    pub const MINTCREAM: Color = Color::from_rgb(245, 255, 250);
    pub const AZURE: Color = Color::from_rgb(240, 255, 255);
    pub const ALICEBLUE: Color = Color::from_rgb(240, 248, 255);
    pub const GHOSTWHITE: Color = Color::from_rgb(248, 248, 255);
    pub const WHITESMOKE: Color = Color::from_rgb(245, 245, 245);
    pub const SEASHELL: Color = Color::from_rgb(255, 245, 238);
    pub const BEIGE: Color = Color::from_rgb(245, 245, 220);
    pub const OLDLACE: Color = Color::from_rgb(253, 245, 230);
    pub const FLORALWHITE: Color = Color::from_rgb(255, 250, 240);
    pub const IVORY: Color = Color::from_rgb(255, 255, 240);
    pub const ANTIQUEWHITE: Color = Color::from_rgb(250, 235, 215);
    pub const LINEN: Color = Color::from_rgb(250, 240, 230);
    pub const LAVENDERBLUSH: Color = Color::from_rgb(255, 240, 245);
    pub const MISTYROSE: Color = Color::from_rgb(255, 228, 225);
    pub const GAINSBORO: Color = Color::from_rgb(220, 220, 220);
    pub const LIGHTGRAY: Color = Color::from_rgb(211, 211, 211);
    pub const SILVER: Color = Color::from_rgb(192, 192, 192);
    pub const DARKGRAY: Color = Color::from_rgb(169, 169, 169);
    pub const GRAY: Color = Color::from_rgb(128, 128, 128);
    pub const DIMGRAY: Color = Color::from_rgb(105, 105, 105);
    pub const LIGHTSLATEGRAY: Color = Color::from_rgb(119, 136, 153);
    pub const SLATEGRAY: Color = Color::from_rgb(112, 128, 144);
    pub const DARKSLATEGRAY: Color = Color::from_rgb(47, 79, 79);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
    pub const BLANK: Color = Color::from_rgba(0, 0, 0, 0);
    pub const RAYWHITE: Color = Color::from_rgb(245, 245, 245);
}
