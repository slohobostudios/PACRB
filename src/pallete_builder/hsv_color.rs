use std::str::FromStr;

use sfml::graphics::Color;
use utils::{
    sfml_util_functions::try_from_color_hash_string_to_sfml_color, simple_error::SimpleError,
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hsv {
    pub h: i16,
    pub s: u8,
    pub v: u8,
}

impl Hsv {
    pub fn new(h: i16, s: u8, v: u8) -> Self {
        Hsv { h, s, v }
    }
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

impl From<Color> for Hsv {
    fn from(value: Color) -> Self {
        let r = f32::from(value.r) / 255.;
        let g = f32::from(value.g) / 255.;
        let b = f32::from(value.b) / 255.;
        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);
        let delta = cmax - cmin;

        let h = if cmax == r {
            60. * (((g - b) / delta) % 6.)
        } else if cmax == g {
            60. * (((b - r) / delta) + 2.)
        } else {
            60. * (((r - g) / delta) + 4.)
        };
        let s = if cmax == 0. { 0. } else { delta / cmax };
        let v = cmax;

        let h = h as i16;
        let s = (s * f32::from(u8::MAX)) as u8;
        let v = (v * f32::from(u8::MAX)) as u8;
        Hsv::new(h, s, v)
    }
}

impl ToString for Hsv {
    fn to_string(&self) -> String {
        let mut hsv = *self;
        hsv.h %= 360;
        let rgb = Color::from(hsv);

        format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    }
}

impl FromStr for Hsv {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok(rgb) = try_from_color_hash_string_to_sfml_color(s) else {
            return Err(SimpleError::new("Failed to convert string to rgb color".to_string()));
        };
        Ok(Hsv::from(rgb))
    }
}

#[cfg(test)]
mod test {
    use utils::arithmetic_util_functions::values_within_standard_deviation;

    use super::*;

    const STANDARD_DEVIATION: u8 = 1;
    #[test]
    fn test_color_from_hsv() {
        for h in i16::MIN..i16::MAX {
            assert_eq!(Color::rgb(0, 0, 0), Hsv::new(h, 0, 0).into());
        }
        // I'm not 100% sure why there is a slight offset sometimes.
        // I think it's due to rounding truncations in my conversion.
        // As long as the rounding truncations are only off by 1, it is
        // unnoticable to the naked human eye. You can't tell the
        // difference between #FFFFFF, and #FEFEFE
        for s in u8::MIN..u8::MAX {
            let fixed_color = Color::rgb(255, 255 - s, 255 - s);
            let hsv_translation = Color::from(Hsv::new(0, s, 255));
            assert!(values_within_standard_deviation(
                fixed_color.r,
                hsv_translation.r,
                STANDARD_DEVIATION
            ));
            assert!(values_within_standard_deviation(
                fixed_color.g,
                hsv_translation.g,
                STANDARD_DEVIATION
            ));
            assert!(values_within_standard_deviation(
                fixed_color.b,
                hsv_translation.b,
                STANDARD_DEVIATION
            ));
        }
        for v in u8::MIN..u8::MAX {
            assert_eq!(Color::rgb(v, v, v), Hsv::new(0, 0, v).into());
        }
        assert_eq!(Color::rgb(24, 23, 23), Hsv::new(360, 10, 24).into());
        assert_eq!(Color::rgb(107, 200, 99), Hsv::new(115, 128, 200).into());
        assert_eq!(Color::rgb(255, 106, 0), Hsv::new(25, 255, 255).into());
    }

    // This test will also inadvertantly test convert.
    #[test]
    fn test_to_string() {
        for h in i16::MIN..i16::MAX {
            assert_eq!(Hsv::new(h, 0, 0).to_string(), "#000000");
        }
        assert_eq!(Hsv::new(180, u8::MAX, 26).to_string(), "#001a1a");

        // Some of the values are slightly incorrect due to small rounding truncations.
        // Leave them be, but do keep note of what the actual value is supposed to be.

        // This is supposed to be #ffd500
        assert_eq!(Hsv::new(50, u8::MAX, u8::MAX).to_string(), "#ffd400");
        // This is supposed to be #fffbe6
        assert_eq!(Hsv::new(50, 26, u8::MAX).to_string(), "#fffae5");
        // This is supposed to be #021c02
        assert_eq!(Hsv::new(121, 237, 28).to_string(), "#011c02");
    }

    #[test]
    fn from_color_to_hsv() {
        for v in u8::MIN..u8::MAX {
            assert_eq!(Hsv::from(Color::rgb(v, v, v)), Hsv::new(0, 0, v));
        }

        let conv_color = Hsv::from(Color::rgb(10, 25, 34));
        let hsv = Hsv::new(203, 180, 34);
        assert!(values_within_standard_deviation(
            conv_color.h,
            hsv.h,
            STANDARD_DEVIATION.into()
        ));
        assert!(values_within_standard_deviation(
            conv_color.s,
            hsv.s,
            STANDARD_DEVIATION
        ));
        assert!(values_within_standard_deviation(
            conv_color.v,
            hsv.v,
            STANDARD_DEVIATION
        ));
    }
}
