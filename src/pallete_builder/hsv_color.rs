use sfml::graphics::Color;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hsv {
    pub h: i16,
    pub s: u8,
    pub v: u8,
}

impl From<Hsv> for Color {
    fn from(hsv: Hsv) -> Self {
        let h = hsv.h % 360;
        let s = f32::from(hsv.s) / 255f32;
        let v = f32::from(hsv.v) / 255f32;

        let c = s * v;
        let x = c * (1. - (((f32::from(h) / 60.) % 2.) - 1.).abs());
        let m = v - c;

        let (r, g, b) = if h < 60 {
            (c, x, 0.)
        } else if h < 120 {
            (x, c, 0.)
        } else if h < 180 {
            (0., c, x)
        } else if h < 240 {
            (0., x, c)
        } else if h < 300 {
            (x, 0., c)
        } else {
            (c, 0., x)
        };
        let r = ((r + m) * 255.) as u8;
        let g = ((g + m) * 255.) as u8;
        let b = ((b + m) * 255.) as u8;
        Color::rgb(r, g, b)
    }
}
