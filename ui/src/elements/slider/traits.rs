use sfml::graphics::IntRect;
use sfml::{
    system::{Vector2, Vector2f, Vector2i},
    window::Event as SFMLEvent,
};
use std::fmt::Debug;
use std::ops::Deref;

use crate::elements::traits::ActionableElement;
use crate::events::Event;
use crate::ui_settings::controls::possible_binds::PossibleBinds;
use crate::ui_settings::controls::possible_inputs::PossibleInputs;
use crate::ui_settings::UISettings;

pub trait Slider: ActionableElement + Debug {
    fn slider_global_bounds(&mut self) -> IntRect;
    fn is_dragging(&self) -> bool;
    fn min_slider_value(&mut self) -> Vector2f;
    fn max_slider_value(&mut self) -> Vector2f;
    /// This function sets the slider's position based on a percentage value. 0 = 0%, u16::MAX =
    /// 100%
    fn set_slider_position_by_percent(&mut self, slider_position_percentage: Vector2<u16>) {
        let min_slider_value = self.min_slider_value();
        let max_slider_value = self.max_slider_value();
        self.set_current_slider_value(
            (slider_position_percentage
                .into_other::<f32>()
                .cwise_mul(max_slider_value - min_slider_value))
                / f32::from(u16::MAX)
                + min_slider_value,
        )
    }

    /// This functions sets the slider's new position based on the slider value. It also sets the
    /// current slider value to a the new slider value if it is in range.
    fn set_current_slider_value(&mut self, new_slider_value: Vector2f);

    fn box_clone(&self) -> Box<dyn Slider>;

    /// This function sets the slider's position based on the cursor coords
    fn set_slider_position_by_cursor_coords(&mut self, cursor: Vector2i) {
        let slider_gb = self.slider_global_bounds();
        let slider_size = slider_gb.size();
        let mut mouse_relative_pos = cursor - slider_gb.position();
        mouse_relative_pos.x = if mouse_relative_pos.x > slider_size.x {
            slider_size.x
        } else if mouse_relative_pos.x < 0 {
            0
        } else {
            mouse_relative_pos.x
        };
        mouse_relative_pos.y = if mouse_relative_pos.y > slider_size.y {
            slider_size.y
        } else if mouse_relative_pos.y < 0 {
            0
        } else {
            mouse_relative_pos.y
        };

        self.set_slider_position_by_percent(
            ((mouse_relative_pos * i32::from(u16::MAX)).cwise_div(slider_gb.size())).as_other(),
        )
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool)
    where
        Self: Sized,
    {
        fn bind_pressed(self_: &mut dyn Slider, ui_settings: &UISettings) -> (Vec<Event>, bool) {
            let prev_event = self_.triggered_event();
            self_.bind_pressed(ui_settings.cursor_position);
            if prev_event != self_.triggered_event() {
                (vec![self_.triggered_event()], true)
            } else {
                Default::default()
            }
        }
        fn bind_released(self_: &mut dyn Slider, ui_settings: &UISettings) -> (Vec<Event>, bool) {
            let prev_event = self_.triggered_event();
            self_.bind_released(ui_settings.cursor_position);
            if prev_event != self_.triggered_event() {
                (vec![self_.triggered_event()], true)
            } else {
                Default::default()
            }
        }
        self.set_hover(ui_settings.cursor_position);
        match event {
            SFMLEvent::MouseButtonPressed { button, .. }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                bind_pressed(self, ui_settings)
            }
            SFMLEvent::KeyPressed { code, .. }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) =>
            {
                bind_pressed(self, ui_settings)
            }
            SFMLEvent::MouseButtonReleased { button, .. }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                bind_released(self, ui_settings)
            }
            SFMLEvent::KeyReleased { code, .. }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) =>
            {
                bind_released(self, ui_settings)
            }
            SFMLEvent::MouseMoved { x: _, y: _ } if self.is_dragging() => {
                self.set_slider_position_by_cursor_coords(ui_settings.cursor_position);
                (vec![self.triggered_event()], true)
            }
            _ => Default::default(),
        }
    }
}

impl Clone for Box<dyn Slider> {
    fn clone(&self) -> Self {
        Slider::box_clone(self.deref())
    }
}
