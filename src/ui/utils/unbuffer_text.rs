use sfml::{
    graphics::{Font, Text, Transformable},
    system::Vector2f,
};

pub fn create_unbuffered_text<'a>(
    font: &'a sfml::SfBox<Font>,
    string: &str,
    font_size: u32,
) -> Text<'a> {
    let mut text = Text::new(string, font, font_size);
    text.set_origin(text.local_bounds().position());
    text
}

pub fn create_unbuffered_text_with_position<'a>(
    font: &'a sfml::SfBox<Font>,
    string: &str,
    font_size: u32,
    position: Vector2f,
) -> Text<'a> {
    let mut text = create_unbuffered_text(font, &string, font_size);
    text.set_position(position);
    text
}
