use sfml::system::Vector2f;

use crate::elements::slider::quad_color_picker::QuadColorPickerSync;

pub type SyncId = u16;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Syncs {
    Boolean(bool),
    Numerical(f32),
    Vector2f(Vector2f),
    String(String),
    QuadColorPicker(QuadColorPickerSync),
    #[default]
    Null,
}

macro_rules! ui_syncs_not_synced_str(() => {"{:?} is not the same sync as {:?}"});
pub(crate) use ui_syncs_not_synced_str;
