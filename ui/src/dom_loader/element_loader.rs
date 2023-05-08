use crate::elements::Element;

use super::{
    background_loader::background_loader, button_loader::button_loader, div_loader::div_loader,
    grid_loader::grid_loader, image_loader::image_loader,
    missing_texture_loader::missing_texture_loader, sets_loader::sets_loader,
    slider_loader::slider_loader, text_loader::text_loader, textbox_loader::textbox_loader,
};
use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use std::error::Error;
use tracing::error;
use utils::{resource_manager::ResourceManager, simple_error::SimpleError};

fn print_error_and_return_missing_texture(
    resource_manager: &ResourceManager,
    error: Box<dyn Error>,
    ele: &MinidomElement,
) -> Element {
    error!(
        "
ui::pages::loader::element_loader::element_loader: Error parsing {}: {:#?}\n\n
Element in question: {:#?}\n
",
        ele.name(),
        error,
        ele
    );
    Element::MissingTexture(missing_texture_loader(resource_manager, ele))
}

/// This function abstracts which exact element to load
pub fn element_loader(
    resource_manager: &ResourceManager,
    ele: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Element {
    match ele.name() {
        "Button" => {
            match button_loader(
                resource_manager,
                ele,
                default_scale,
                default_font_size,
                default_color,
            ) {
                Ok(v) => Element::Button(v),
                Err(e) => print_error_and_return_missing_texture(resource_manager, e, ele),
            }
        }
        "Slider" => {
            match slider_loader(resource_manager, ele, default_scale, default_font_size, default_color) {
                Ok(v) => Element::Slider(v),
                Err(e) => print_error_and_return_missing_texture(resource_manager, e, ele),
            }
        }
        "TextBox" => {
            match textbox_loader(resource_manager, ele, default_scale, default_font_size, default_color) {
                Ok(v) => Element::TextBox(v),
                Err(e) => print_error_and_return_missing_texture(resource_manager, e, ele),
            }
        }
        "Div" => Element::Div(div_loader(
            resource_manager,
            ele,
            default_scale,
            default_font_size,
            default_color
        )),
        "Grid" => match grid_loader(
            resource_manager,
            ele,
            default_scale,
            default_font_size,
            default_color,
        ) {
            Ok(v) => Element::Grid(v),
            Err(e) => print_error_and_return_missing_texture(resource_manager, e, ele),
        },
        "Sets" => Element::Sets(sets_loader(
            resource_manager,
            ele,
            default_scale,
            default_font_size,
            default_color
        )),
        "Background" => match background_loader(resource_manager, ele, default_scale, default_font_size, default_color) {
            Ok(v) => Element::Background(v),
            Err(e) => print_error_and_return_missing_texture(resource_manager, e, ele),
        },
        "Text" => Element::Text(text_loader(resource_manager, ele, default_font_size, default_color)),
        "Image" => match image_loader(resource_manager, ele, default_scale) {
            Ok(v) => Element::Image(v),
            Err(e) => print_error_and_return_missing_texture(resource_manager, e, ele)
        },
        "Empty" => Element::Empty(()),
        _ => print_error_and_return_missing_texture(resource_manager,
            Box::new(SimpleError::new(format!(
                "ui::pages::loader::element_loader::element_loader: No dom element labeled {} exists",
                ele.name()
            ))),
            ele,
        ),
    }
}
