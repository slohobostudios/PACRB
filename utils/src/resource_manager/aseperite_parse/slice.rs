use sfml::graphics::Color;

use super::slice_key::SliceKey;

#[derive(Debug, Clone)]
pub struct Slice {
    pub name: String,
    pub color: Color,
    pub keys: Vec<SliceKey>,
}

impl Default for Slice {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            name: Default::default(),
            keys: Default::default(),
        }
    }
}
/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_color_hex() {
        // rgb tests
        assert_eq!(
            Slice::parse_color_hex("#FFFFFF").unwrap(),
            Color::rgb(255, 255, 255)
        );
        assert_eq!(
            Slice::parse_color_hex("#000000").unwrap(),
            Color::rgb(0, 0, 0)
        );
        assert_eq!(
            Slice::parse_color_hex("#3780B2").unwrap(),
            Color::rgb(55, 128, 178)
        );

        // rgba tests
        assert_eq!(
            Slice::parse_color_hex("#FFFFFFFF").unwrap(),
            Color::rgba(255, 255, 255, 255)
        );
        assert_eq!(
            Slice::parse_color_hex("#00000000").unwrap(),
            Color::rgba(0, 0, 0, 0)
        );
        assert_eq!(
            Slice::parse_color_hex("#3780B2A1").unwrap(),
            Color::rgba(55, 128, 178, 161)
        );
    }
}*/
